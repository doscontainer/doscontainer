use audio::AudioDevice;
use cpu::Cpu;
use error::HwSpecError;
use regex::Regex;

mod audio;
mod cpu;
mod error;
mod storage;

/// This struct represents the hardware configuration of an MS-DOS
/// compatible PC system.
pub struct HwSpec {
    cpu: Cpu,
    ram: u32, // RAM size in bytes
    audio: Vec<AudioDevice>,
}

impl HwSpec {
    pub fn set_ram(&mut self, ram: &str) -> Result<(), HwSpecError> {
        if Self::valid_ram_amount(ram) {
            println!("YUP");
            Ok(())
        } else {
            Err(HwSpecError::InvalidCpu)
        }
    }

    /// Validator for the RAM amount. Tiny function, but broken out for clarity.
    fn valid_ram_amount(ram: &str) -> bool {
        let re = Regex::new(r"^\d+(?i)(KB|MB)$").unwrap();
        re.is_match(ram)
    }
}
