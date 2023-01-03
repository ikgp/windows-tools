use std::time::Duration;
use windows::Win32::Graphics::Gdi::{ChangeDisplaySettingsA, EnumDisplaySettingsA, DEVMODEA};
use windows::Win32::Media::Audio::Endpoints::IAudioEndpointVolume;
use windows::Win32::Media::Audio::{IMMDeviceEnumerator, MMDeviceEnumerator};

use windows::core::GUID;
use windows::Win32::System::Com::{CoInitialize, CoCreateInstance, CLSCTX_ALL};
use windows::Win32::System::Diagnostics::Debug::Beep;

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
