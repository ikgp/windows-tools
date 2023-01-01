// Parts of this code is based on Deno. https://github.com/denoland/deno/tree/f729576b2db2aa6ce000a598ad2e45533f686213
// Deno is available under the following license:
// Copyright 2018-2022 the Deno authors. All rights reserved. MIT license.

use crate::beep;
use deno_core::error::AnyError;
use deno_core::url::Url;
use deno_core::v8;
use deno_core::FsModuleLoader;
use deno_runtime::deno_broadcast_channel::InMemoryBroadcastChannel;
use deno_runtime::deno_web::BlobStore;
use deno_runtime::permissions::Permissions;
use deno_runtime::worker::MainWorker;
use deno_runtime::worker::WorkerOptions;
use deno_runtime::BootstrapOptions;
use std::path::Path;
use std::rc::Rc;
use std::sync::Arc;
use std::time::Duration;

fn throw_error(scope: &mut v8::HandleScope, message: impl AsRef<str>) {
    let message = v8::String::new(scope, message.as_ref()).unwrap();
    let exception = v8::Exception::error(scope, message);
    scope.throw_exception(exception);
}

fn throw_type_error(scope: &mut v8::HandleScope, message: impl AsRef<str>) {
    let message = v8::String::new(scope, message.as_ref()).unwrap();
    let exception = v8::Exception::type_error(scope, message);
    scope.throw_exception(exception);
}

fn get_error_class_name(e: &AnyError) -> &'static str {
    deno_runtime::errors::get_error_class_name(e).unwrap_or("Error")
}

async fn execute_js(main_module: &Url) -> Result<(), AnyError> {
    let module_loader = Rc::new(FsModuleLoader);
    let create_web_worker_cb = Arc::new(|_| {
        todo!("Web workers are not supported yet!");
    });
    let web_worker_event_cb = Arc::new(|_| {
        todo!("Web workers are not supported yet!");
    });

    let options = WorkerOptions {
        bootstrap: BootstrapOptions {
            args: vec![],
            cpu_count: std::thread::available_parallelism()
                .map(|p| p.get())
                .unwrap_or(1),
            debug_flag: false,
            enable_testing_features: false,
            locale: deno_core::v8::icu::get_language_tag(),
            location: None,
            no_color: false,
            is_tty: false,
            runtime_version: "x".to_string(),
            ts_version: "x".to_string(),
            unstable: false,
            user_agent: "IGKP".to_string(),
            inspect: false,
        },
        extensions: vec![],
        startup_snapshot: None,
        unsafely_ignore_certificate_errors: None,
        root_cert_store: None,
        seed: None,
        source_map_getter: None,
        format_js_error_fn: None,
        web_worker_preload_module_cb: web_worker_event_cb.clone(),
        web_worker_pre_execute_module_cb: web_worker_event_cb,
        create_web_worker_cb,
        maybe_inspector_server: None,
        should_break_on_first_statement: false,
        should_wait_for_inspector_session: false,
        module_loader,
        npm_resolver: None,
        get_error_class_fn: Some(&get_error_class_name),
        cache_storage_dir: None,
        origin_storage_dir: None,
        blob_store: BlobStore::default(),
        broadcast_channel: InMemoryBroadcastChannel::default(),
        shared_array_buffer_store: None,
        compiled_wasm_module_store: None,
        stdio: Default::default(),
    };

    let permissions = Permissions::allow_all();

    let mut worker = MainWorker::bootstrap_from_options(main_module.clone(), permissions, options);
    // Insert a custom native function into the worker's global scope
    let isolate = &mut worker.js_runtime;
    {
        let scope = &mut isolate.handle_scope();
        let context = v8::Context::new(scope);
        let global = context.global(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        // KantBeep(freq: number, duration: number)
        let my_func_key = v8::String::new(scope, "KantBeep").unwrap();
        let my_func_templ = v8::FunctionTemplate::new(
            scope,
            |scope: &mut v8::HandleScope,
             args: v8::FunctionCallbackArguments,
             _rv: v8::ReturnValue| {
                if args.length() != 2 {
                    return throw_type_error(scope, "Invalid arguments");
                }
                let freq = args.get(0);
                let duration = args.get(1);
                let Some(freq) = freq.uint32_value(scope) else {
                    return throw_type_error(scope, "Frequency needs to be a positive (unsigned) 32 bit integer");
                };
                let Some(duration) = duration.uint32_value(scope) else {
                    return throw_type_error(scope, "Duration needs to be a positive (unsigned) 32 bit integer");
                };
                if let Err(beep_error) = beep(freq, Duration::from_millis(duration.into())) {
                    return throw_error(scope, beep_error.to_string());
                }
            },
        );
        let my_func_val = my_func_templ.get_function(scope).unwrap();

        global.set(scope, my_func_key.into(), my_func_val.into());
    }
    worker.run_event_loop(false).await?;
    Ok(())
}

pub async fn execute_js_from_path(path: &Path) -> Result<(), AnyError> {
    let main_module = deno_core::resolve_path(&path.to_string_lossy())?;
    execute_js(&main_module).await?;
    Ok(())
}

pub async fn execute_js_from_url(url: &str) -> Result<(), AnyError> {
    let url = Url::parse(url)?;
    execute_js(&url).await?;
    Ok(())
}
