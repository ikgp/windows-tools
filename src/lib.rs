use std::time::Duration;

pub mod js;
pub mod songs;
#[cfg(windows)]
pub mod windows;

pub fn beep(frequency: u32, duration: Duration) -> anyhow::Result<()> {
    #[cfg(windows)]
    {
        windows::beep(frequency, duration)?;
    }
    #[cfg(unix)]
    {
        use std::thread::sleep;

        beep::beep(frequency.try_into()?)?;
        // We need to sleep here, and then beepp with a frequency of 0, because beep::beep does not block
        sleep(duration);
        // TODO: Allow manually ending the beep and don't stop at the end
        beep::beep(0)?;
    }
    Ok(())
}