use audio::AudioDevice;
use cpu::Cpu;

mod audio;
mod cpu;
mod error;
mod storage;

/// This struct represents the hardware configuration of an MS-DOS
/// compatible PC system.
pub struct HwState {
    cpu: Cpu,
    ram: u32, // RAM size in bytes
    audio: Vec<AudioDevice>,
}
