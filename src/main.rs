use chrono::{Datelike, Timelike};
use kant_tools::beep;
use kant_tools::songs::last_christmas;
use std::thread::sleep;

fn main() {
    #[cfg(windows)]
    {
        use kant_tools::windows::init;
        init();
    }
    #[cfg(windows)]
    {
        use kant_tools::windows::set_volume;
        if let Err(e) = set_volume(70) {
            println!("Failed to set volume: {e}");
        };
    }
    #[cfg(windows)]
    {
        use kant_tools::windows as _windows;
        use windows::Win32::Graphics::Gdi::DEVMODEA;
        let reinit_result = _windows::force_reinit_screen();
        println!("Screen reinit returned {}", reinit_result);
        let supported_resolutions =
            _windows::get_screen_settings().expect("Failed to get screen settings");
        // TODO: Optimize this code
        let mut highest_resolution: DEVMODEA = DEVMODEA::default();
        // Print all resolutions
        let amount_of_resolutions = supported_resolutions.len();
        for resolution in supported_resolutions {
            println!(
                "{}x{} ({} bit)",
                resolution.dmPelsWidth, resolution.dmPelsHeight, resolution.dmBitsPerPel
            );
            if (resolution.dmPelsWidth * resolution.dmPelsHeight)
                > (highest_resolution.dmPelsWidth * highest_resolution.dmPelsHeight)
            {
                highest_resolution = resolution;
            } else if (resolution.dmPelsWidth * resolution.dmPelsHeight)
                == (highest_resolution.dmPelsWidth * highest_resolution.dmPelsHeight)
            {
                if resolution.dmBitsPerPel > highest_resolution.dmBitsPerPel {
                    highest_resolution = resolution;
                } else {
                    // DEBUG: Print flags
                    println!("dmDisplayFrequency: {:?}", resolution.dmDisplayFrequency);
                    println!("{:#x}", resolution.dmFields.0);
                    todo!();
                }
            }
        }

        if amount_of_resolutions == 1 {
            eprintln!("Error: Only one resolution available!");
        }

        println!(
            "Highest resolution: {}x{} ({} bit)",
            highest_resolution.dmPelsWidth,
            highest_resolution.dmPelsHeight,
            highest_resolution.dmBitsPerPel
        );
        // Set the highest resolution
        _windows::set_device_mode(highest_resolution).expect("Failed to set screen settings");
    }

    println!("Last christmas, I gave you my heart...");
    let now = chrono::Local::now();
    // This breaks 2028, but I don't care right now
    if now.month() == 12 && [21, 22, 23, 24].contains(&now.day()) {
        if let Err(e) = last_christmas() {
            println!("Failed to play last christmas: {e}");
        };
    }

    // If it's 17:20 or later play 3 second-long beeps with 200ms in between, then shut down
    if now.hour() >= 17 && now.minute() >= 20 {
        for _ in 0..3 {
            beep(650, std::time::Duration::from_secs(1)).unwrap();
            sleep(std::time::Duration::from_millis(200));
        }
        #[cfg(windows)]
        {
            use kant_tools::windows::shutdown;
            if let Err(e) = shutdown(std::time::Duration::from_millis(0), "") {
                eprintln!("{}", e);
                loop {
                    beep(650, std::time::Duration::from_millis(500)).unwrap();
                    sleep(std::time::Duration::from_millis(2000));
                }
            }
        }
    }
    // Otherwise, sleep until 17:20 (not 17:30 because of the 10 minute grace period)
    else {
        //let until = chrono::Local::now().with_hour(17).unwrap().with_minute(20).unwrap();
        let until = chrono::Local::now().with_hour(13).unwrap().with_minute(47).unwrap();
        if now > until {
            panic!("Time is in the past!");
        }
        let duration = until - now;
        println!("Sleeping for {} seconds", duration.num_seconds());
        sleep(std::time::Duration::from_secs(duration.num_seconds() as u64));
        // Schedule a shutdown for 17:30
        #[cfg(windows)]
        {
            use kant_tools::windows::shutdown;
            beep(650, std::time::Duration::from_secs(1)).unwrap();
            if let Err(e) = shutdown(std::time::Duration::from_secs(60 * 10), "Die Schul-PCs sind ab 17:30 gesperrt. Bitte speichern Sie alle geöffneten Dokumente und schließen Sie alle Programme, um Datenverlust zu vermeiden!") {
                eprintln!("{}", e);
                loop {
                    beep(650, std::time::Duration::from_millis(500)).unwrap();
                    sleep(std::time::Duration::from_millis(2000));
                }
            }
        }
        // Wait until 17:35, then play 3 beeps, print another warning for 2 minutes, then, wait 5 minutes, then force a shutdown with 0 seconds delay
        sleep(std::time::Duration::from_secs(60 * 5));
        for _ in 0..3 {
            beep(650, std::time::Duration::from_secs(1)).unwrap();
            sleep(std::time::Duration::from_millis(200));
        }
        #[cfg(windows)]
        {
            use kant_tools::windows::shutdown;
            if let Err(e) = shutdown(std::time::Duration::from_secs(60 * 2), "Die Schul-PCs sind ab 17:30 gesperrt. Bitte speichern Sie alle geöffneten Dokumente und schließen Sie alle Programme, um Datenverlust zu vermeiden!") {
                eprintln!("{}", e);
            }
        }
        sleep(std::time::Duration::from_secs(60 * 5));
        #[cfg(windows)]
        {
            use kant_tools::windows::shutdown;
            if let Err(e) = shutdown(std::time::Duration::from_secs(0), "") {
                eprintln!("{}", e);
                loop {
                    beep(650, std::time::Duration::from_millis(500)).unwrap();
                    sleep(std::time::Duration::from_millis(2000));
                }
            }
        }

    }
}
