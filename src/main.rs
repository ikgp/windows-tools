use kant_tools::songs::last_christmas;

#![windows_subsystem = "windows"]
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
        if let Err(e) = last_christmas() {
            println!("Failed to play last christmas: {e}");
        };
    }
}
