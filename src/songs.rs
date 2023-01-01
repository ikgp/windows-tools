use crate::windows::beep;
use std::thread::sleep;
use std::time::Duration;

pub fn last_christmas() -> anyhow::Result<()> {
    beep(659, Duration::from_millis(900))?;
    beep(659, Duration::from_millis(600))?;
    beep(587, Duration::from_millis(600))?;
    beep(440, Duration::from_millis(300))?;
    beep(659, Duration::from_millis(300))?;
    beep(659, Duration::from_millis(300))?;
    beep(740, Duration::from_millis(300))?;
    beep(587, Duration::from_millis(900))?;
    beep(494, Duration::from_millis(300))?;
    beep(494, Duration::from_millis(300))?;
    beep(659, Duration::from_millis(300))?;
    beep(659, Duration::from_millis(300))?;
    beep(740, Duration::from_millis(600))?;
    beep(587, Duration::from_millis(900))?;
    beep(494, Duration::from_millis(300))?;
    beep(554, Duration::from_millis(300))?;
    beep(587, Duration::from_millis(300))?;
    beep(554, Duration::from_millis(300))?;
    beep(494, Duration::from_millis(1200))?;
    sleep(Duration::from_millis(300));
    beep(740, Duration::from_millis(900))?;
    beep(659, Duration::from_millis(1200))?;
    beep(494, Duration::from_millis(300))?;
    beep(740, Duration::from_millis(300))?;
    beep(784, Duration::from_millis(300))?;
    beep(740, Duration::from_millis(300))?;
    beep(659, Duration::from_millis(1200))?;
    beep(587, Duration::from_millis(300))?;
    beep(554, Duration::from_millis(300))?;
    beep(587, Duration::from_millis(300))?;
    beep(587, Duration::from_millis(300))?;
    beep(554, Duration::from_millis(600))?;
    beep(587, Duration::from_millis(600))?;
    beep(554, Duration::from_millis(600))?;
    beep(440, Duration::from_millis(1200))?;
    Ok(())
}
