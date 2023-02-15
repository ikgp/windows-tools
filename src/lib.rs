use std::time::Duration;

pub mod songs;
#[cfg(unix)]
pub mod unix;
//#[cfg(windows)]
pub mod windows;

pub fn beep(frequency: u32, duration: Duration) -> anyhow::Result<()> {
    #[cfg(windows)]
    {
        windows::beep(frequency, duration)?;
    }
    #[cfg(unix)]
    {
        unix::beep(frequency, duration)?;
    }
    Ok(())
}
