use std::str::FromStr;

use audio::{AudioDevice, AudioDeviceType};
use byte_unit::Byte;
use cpu::Cpu;
use error::HwSpecError;
use video::VideoDevice;

mod audio;
mod cpu;
mod error;
mod storage;
mod tests;
mod video;

/// Represents the hardware configuration of an MS-DOS compatible PC system.
pub struct HwSpec {
    cpu: Cpu,
    ram: u32, // RAM size in bytes
    audio: Vec<AudioDevice>,
    video: Vec<VideoDevice>,
}

impl HwSpec {
    pub fn add_audio_device(&mut self, device: AudioDevice) -> Result<(), HwSpecError> {
        // We already have the device you're trying to add.
        // Mind you, you can still have multiple instances of the same device just not
        // 100% identical ones.
        if self.audio.contains(&device) {
            return Err(HwSpecError::DuplicateAudioDevice);
        }
        self.audio.push(device);
        Ok(())
    }

    pub fn audio(&self) -> &[AudioDevice] {
        &self.audio
    }

    pub fn audio_device(&self, devicetype: AudioDeviceType) -> Vec<&AudioDevice> {
        self.audio.iter().filter(|d| d.device_type() == &devicetype).collect()
    }

    pub fn set_cpu(&mut self, cpu: &str) -> Result<(), HwSpecError> {
        self.cpu = Cpu::from_str(cpu)?;
        Ok(())
    }

    pub fn set_ram(&mut self, ram: &str) -> Result<(), HwSpecError> {
        const IGNORE_CASE: bool = true;
        let amount =
            Byte::parse_str(ram, IGNORE_CASE).map_err(|_| HwSpecError::InvalidRamString)?;
        self.ram = amount
            .try_into()
            .map_err(|_| HwSpecError::TooMuchRamSpecified)?;
        Ok(())
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn ram(&self) -> u32 {
        self.ram
    }
}
