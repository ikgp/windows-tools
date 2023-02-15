use std::time::Duration;
use windows::core::{GUID, HSTRING, PCWSTR, PCSTR};
use windows::Win32::Devices::Display::{
    GetDisplayConfigBufferSizes, QueryDisplayConfig, SetDisplayConfig, DISPLAYCONFIG_MODE_INFO,
    DISPLAYCONFIG_PATH_INFO, DISPLAYCONFIG_TOPOLOGY_ID,
};
use windows::Win32::Graphics::Gdi::{
    ChangeDisplaySettingsA, EnumDisplaySettingsA, DEVMODEA, QDC_ALL_PATHS, SDC_APPLY,
    SDC_FORCE_MODE_ENUMERATION, SDC_SAVE_TO_DATABASE, SDC_USE_SUPPLIED_DISPLAY_CONFIG,
};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator};
use windows::Win32::System::Com::{CoCreateInstance, CoInitialize, CLSCTX_ALL};
use windows::Win32::System::Diagnostics::Debug::Beep;
use windows::Win32::System::Shutdown::{
    InitiateSystemShutdownExW, SHTDN_REASON_FLAG_PLANNED, SHTDN_REASON_MAJOR_OTHER,
};
use windows::Win32::Graphics::Printing::PRINTER_INFO_2A;
use windows::Win32::Foundation::GetLastError;

// Forces Windows to reinit display settings with SetDisplayConfig and the provided flags
pub fn force_reinit_screen() -> i32 {
    assert_eq!(std::mem::size_of::<DISPLAYCONFIG_PATH_INFO>(), 72);
    assert_eq!(std::mem::size_of::<DISPLAYCONFIG_MODE_INFO>(), 64);
    let mut path_count = 0;
    let mut mode_count = 0;
    let result =
        unsafe { GetDisplayConfigBufferSizes(QDC_ALL_PATHS, &mut path_count, &mut mode_count) };
    println!("GetDisplayConfigBufferSizes returned {}", result);
    let mut path_array = Vec::with_capacity(path_count as usize);
    let mut mode_array = Vec::with_capacity(mode_count as usize);
    println!("Got {} display paths", path_count);
    println!("Got {} display modes", mode_count);
    let result = unsafe {
        QueryDisplayConfig(
            QDC_ALL_PATHS,
            &mut path_count,
            path_array.as_mut_ptr(),
            &mut mode_count,
            mode_array.as_mut_ptr(),
            ::core::mem::transmute(::core::ptr::null::<DISPLAYCONFIG_TOPOLOGY_ID>()),
        )
    };
    println!("Got {} display paths", path_count);
    println!("Got {} display modes", mode_count);
    println!("QueryDisplayConfig returned {}", result);
    let flags = SDC_FORCE_MODE_ENUMERATION
        | SDC_APPLY
        | SDC_USE_SUPPLIED_DISPLAY_CONFIG
        | SDC_SAVE_TO_DATABASE;

    let result = unsafe { SetDisplayConfig(Some(&path_array), Some(&mode_array), flags) };
    result
}

pub fn init() -> windows::core::Result<()> {
    unsafe { CoInitialize(None) }
}
/// Plays a beep sound
/// duration is the amount to play in ms, this function will not return before the beep is finished playing
fn beep_ms(frequency: u32, duration: u32) -> windows::core::Result<()> {
    let ret_val = unsafe { Beep(frequency, duration) };
    return ret_val.ok();
}

pub fn beep(frequency: u32, duration: Duration) -> anyhow::Result<()> {
    let duration_ms = u32::try_from(duration.as_millis())?;
    beep_ms(frequency, duration_ms)?;
    Ok(())
}

pub fn set_device_mode(mut dev_mode: DEVMODEA) -> anyhow::Result<()> {
    let ret_val = unsafe {
        ChangeDisplaySettingsA(
            Some(&mut dev_mode),
            windows::Win32::Graphics::Gdi::CDS_TYPE(0),
        )
    };
    // TODO: Remove this debug print
    println!("ChangeDisplaySettingsA returned {}", ret_val.0);
    Ok(())
}

pub fn get_screen_settings() -> anyhow::Result<Vec<DEVMODEA>> {
    let mut dev_mode = DEVMODEA::default();
    let mut settings = Vec::new();
    let mut i = 0;
    loop {
        let ret_val = unsafe {
            EnumDisplaySettingsA(
                None,
                windows::Win32::Graphics::Gdi::ENUM_DISPLAY_SETTINGS_MODE(i),
                &mut dev_mode,
            )
        };
        if ret_val.0 == 0 {
            break;
        }
        settings.push(dev_mode);
        i += 1;
    }
    Ok(settings)
}

pub fn set_volume(volume: u32) -> anyhow::Result<()> {
    let device_enumerator: IMMDeviceEnumerator =
        unsafe { CoCreateInstance(&MMDeviceEnumerator, None, CLSCTX_ALL) }?;
    let default_device = unsafe {
        device_enumerator.GetDefaultAudioEndpoint(
            windows::Win32::Media::Audio::eRender,
            windows::Win32::Media::Audio::eMultimedia,
        )
    }?;
    let volume_control =
        unsafe { default_device.Activate::<IAudioEndpointVolume>(CLSCTX_ALL, None) }?;
    let volume = volume as f32 / 100.0;
    unsafe { volume_control.SetMasterVolumeLevelScalar(volume, &GUID::zeroed()) }?;
    Ok(())
}

pub fn shutdown(delay: Duration, msg: &str) -> anyhow::Result<()> {
    let h_msg = HSTRING::from(msg);
    let msg = PCWSTR::from_raw(h_msg.as_ptr());
    let delay_secs = delay.as_secs().try_into()?;
    let shutdown_result = unsafe {
        InitiateSystemShutdownExW(
            None,
            msg,
            delay_secs,
            true,
            false,
            SHTDN_REASON_FLAG_PLANNED | SHTDN_REASON_MAJOR_OTHER,
        )
    };
    shutdown_result.ok()?;
    Ok(())
}

pub fn list_printers() -> Vec<PRINTER_INFO_2A> {
    let mut printers = Vec::new();
    let level = 2;
    let flags = 0;
    let buffer_size = std::mem::size_of::<PRINTER_INFO_2A>();
    let buffer = unsafe { std::alloc::alloc(std::alloc::Layout::from_size_align(buffer_size, 1).unwrap()) };
    let mut buffer: *mut [u8] = unsafe { std::slice::from_raw_parts_mut(buffer, buffer_size) };
    let mut needed = 0;
    let mut returned = 0;
    let mut ret_val = unsafe {
        windows::Win32::Graphics::Printing::EnumPrintersA(
            flags,
            PCSTR::null(),
            level,
            Some(&mut *buffer),
            &mut needed,
            &mut returned,
        )
    };
    // Check if there is any error
    // If yes, print the error and exit
    if ret_val.0 != 0 {
        println!("Error: {}", ret_val.0);
        let details = unsafe { GetLastError() };
        println!("Error details: {}", details.0);
        return printers;
    } else {
        print!("No error");
    }

    while ret_val.0 == 0 {
        unsafe {
            let mut _buffer =std::alloc::alloc(std::alloc::Layout::from_size_align(buffer_size, 1).unwrap());
            buffer = std::slice::from_raw_parts_mut(_buffer, buffer_size);
            ret_val = windows::Win32::Graphics::Printing::EnumPrintersA(
                flags,
                PCSTR::null(),
                level,
                Some(&mut *buffer),
                &mut needed,
                &mut returned,
            );
        }
    }
    for i in 0..returned {
        let printer = unsafe { &*(buffer as *mut PRINTER_INFO_2A).add(i as usize) };
        printers.push(*printer);
    }
    unsafe {
        std::alloc::dealloc(buffer as *mut u8, std::alloc::Layout::from_size_align(buffer_size, 1).unwrap());
    }
    printers

}
