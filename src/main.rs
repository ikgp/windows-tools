use kant_tools::js::execute_js_from_url;
use kant_tools::songs::last_christmas;

#[tokio::main]
async fn main() {
    #[cfg(windows)]
    {
        use kant_tools::windows::set_volume;
        if let Err(e) = set_volume(70) {
            println!("Failed to set volume: {e}");
        };
    }
    println!("Last christmas, I gave you my heart...");
    if let Err(e) = last_christmas() {
        println!("Failed to play last christmas: {e}");
    };
    println!("But the very next day, you gave it away...");
    if let Err(e) = execute_js_from_url("https://raw.githubusercontent.com/ikgp/windows-tools/f094ded46593eef9bb5084b661579aa0a49e8c6c/examples/tetris.js").await {
        println!("Failed to play the tetris theme: {e}");
    };
    #[cfg(windows)]
    {
        use kant_tools::windows as _windows;
        use windows::Win32::Graphics::Gdi::DEVMODEA;
        let supported_resolutions =
            _windows::get_screen_settings().expect("Failed to get screen settings");
        // TODO: Optimize this code
        let mut highest_resolution: DEVMODEA = DEVMODEA::default();
        // Print all resolutions
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
        println!(
            "Highest resolution: {}x{} ({} bit)",
            highest_resolution.dmPelsWidth,
            highest_resolution.dmPelsHeight,
            highest_resolution.dmBitsPerPel
        );
        // Set the highest resolution
        _windows::set_device_mode(highest_resolution).expect("Failed to set screen settings");
    }
}
