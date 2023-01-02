use nix::{self, ioctl_write_int_bad};
use std::fs::{File, OpenOptions};
use std::os::unix::io::AsRawFd;
use std::thread::sleep;
use std::time::Duration;
use anyhow::{Result, bail};
use lazy_static::lazy_static;

pub use nix::Error;

const FILE            : &str = "/dev/console";
const KIOCSOUND       : u64  = 0x4B2F;
const TIMER_FREQUENCY : u32  = 1193180;

lazy_static! {
    static ref DEVICE: Result<File, std::io::Error> =
        OpenOptions::new()
            .append(true)
            .open(FILE);
}

ioctl_write_int_bad!(kiocsound, KIOCSOUND);

/// Play an indefinite tone of a given `hertz`.
pub fn beep_indefinite(hertz: u32) -> Result<()>
{
    let period_in_clock_cycles =
        TIMER_FREQUENCY.checked_div(hertz).unwrap_or(0);

    if let Some(e) = DEVICE.as_ref().err() {
        bail!("Could not open {}: {}", FILE, e);
    }
    let device = DEVICE.as_ref().unwrap();
    unsafe {
        kiocsound(device.as_raw_fd(), period_in_clock_cycles as i32)?;
    }

    Ok(())
}

pub fn beep(frequency: u32, duration: Duration) -> anyhow::Result<()> {
    beep_indefinite(frequency.try_into()?)?;
    // We need to sleep here, and then beepp with a frequency of 0, because beep::beep does not block
    sleep(duration);
    // TODO: Allow manually ending the beep and don't stop at the end in the public API
    beep_indefinite(0)?;
    Ok(())
}
