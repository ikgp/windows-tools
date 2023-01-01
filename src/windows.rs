//use windows::Win32::Graphics::Gdi::ChangeDisplaySettingsExA;
use std::time::Duration;
use windows::Win32::System::Diagnostics::Debug::Beep;

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

// TODO: Allow changing screen resolution
