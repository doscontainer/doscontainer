use audio::AudioDevice;
use byte_unit::Byte;
use cpu::Cpu;
use error::HwSpecError;
use video::VideoDevice;

mod audio;
mod cpu;
mod error;
mod storage;
mod video;

/// This struct represents the hardware configuration of an MS-DOS
/// compatible PC system.
pub struct HwSpec {
    cpu: Cpu,
    ram: u32, // RAM size in bytes
    audio: Vec<AudioDevice>,
    video: Vec<VideoDevice>,
}

impl HwSpec {
    pub fn set_ram(&mut self, ram: &str) -> Result<(), HwSpecError> {
        const IGNORE_CASE: bool = true;
        let amount = Byte::parse_str(ram, IGNORE_CASE).map_err(|_| HwSpecError::InvalidRamString)?;
        self.ram = amount.try_into().map_err(|_| HwSpecError::TooMuchRamSpecified)?;
        Ok(())
    }
}
