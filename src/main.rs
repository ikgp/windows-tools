use kant_tools::songs::last_christmas;
use kant_tools::js::execute_js_from_url;

#[tokio::main]
async fn main() {
    println!("Last christmas, I gave you my heart...");
    if let Err(e) = last_christmas() {
        println!("Failed to play last christmas: {e}");
    };
    println!("But the very next day, you gave it away...");
    if let Err(e) = execute_js_from_url("https://raw.githubusercontent.com/ikgp/windows-tools/1e7fc3e8ffc9fa549190906543d6ade3932c7e30/examples/tetris.js").await {
        println!("Failed to play the tetris theme: {e}");
    };
    #[cfg(windows)]
    {
        use kant_tools::windows as _windows;
        use windows::Win32::Graphics::Gdi::DEVMODEA;
        let supported_resolutions = _windows::get_screen_settings().expect("Failed to get screen settings");
        // TODO: Optimize this code
        let mut highest_resolution: DEVMODEA = DEVMODEA::default();
        // Print all resolutions
        for resolution in supported_resolutions {
            println!("{}x{} ({} bit)", resolution.dmPelsWidth, resolution.dmPelsHeight, resolution.dmBitsPerPel);
            if (resolution.dmPelsWidth * resolution.dmPelsHeight) > (highest_resolution.dmPelsWidth * highest_resolution.dmPelsHeight) {
                highest_resolution = resolution;
            } else if (resolution.dmPelsWidth * resolution.dmPelsHeight) == (highest_resolution.dmPelsWidth * highest_resolution.dmPelsHeight) {
                if resolution.dmBitsPerPel > highest_resolution.dmBitsPerPel {
                    highest_resolution = resolution;
                } else {
                    // DEBUG: Print flags
                    println!("{:#x}", resolution.dmFields.0);
                    todo!();
                }
            }
        }
        println!("Highest resolution: {}x{} ({} bit)", highest_resolution.dmPelsWidth, highest_resolution.dmPelsHeight, highest_resolution.dmBitsPerPel);
        // Set the highest resolution
        _windows::set_device_mode(highest_resolution).expect("Failed to set screen settings");
    }
}
