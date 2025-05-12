use std::fmt;
use std::path::Path;
use std::str::FromStr;

use audio::{AudioDevice, AudioDeviceType};
use byte_unit::Byte;
use config::Config;
use config::File;
use config::FileFormat;
use cpu::Cpu;
use error::HwSpecError;
use serde::{Deserialize, Deserializer};
use serde_with::OneOrMany;
use serde_with::serde_as;
use video::VideoDevice;

mod audio;
mod cpu;
mod error;
mod storage;
mod tests;
mod video;

/// Represents the hardware configuration of an MS-DOS compatible PC system.
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct HwSpec {
    // #[serde(deserialize_with = "deserialize_cpu")]
    cpu: Cpu,
    #[serde(deserialize_with = "deserialize_ram")]
    ram: u32,
    #[serde(default)]
    audio: Vec<AudioDevice>,
    #[serde_as(as = "OneOrMany<_>")]
    video: Vec<VideoDevice>,
}

impl Default for HwSpec {
    fn default() -> Self {
        HwSpec {
            cpu: Cpu::from_str("8088").unwrap(),
            ram: 0,
            audio: Vec::new(),
            video: Vec::new(),
        }
    }
}

impl HwSpec {
    /// Adds an audio device to the system.
    ///
    /// This method inserts a new [`AudioDevice`] into the list of audio devices.
    /// If an identical device is already present, the addition will fail.
    ///
    /// Note: It is possible to have multiple instances of the same type of device,
    /// as long as they are not completely identical (e.g., two different Sound Blaster cards).
    ///
    /// # Arguments
    ///
    /// * `device` - The [`AudioDevice`] to add to the system.
    ///
    /// # Errors
    ///
    /// Returns a [`HwSpecError::DuplicateAudioDevice`] if the exact same device is already present.
    ///
    pub fn add_audio_device(&mut self, device: AudioDevice) -> Result<(), HwSpecError> {
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
        self.audio
            .iter()
            .filter(|d| d.device_type() == &devicetype)
            .collect()
    }

    /// Adds a video device to the system.
    ///
    /// This method inserts a new [`VideoDevice`] into the list of video devices.
    /// If an identical device is already present, the addition will fail.
    ///
    /// # Arguments
    ///
    /// * `device` - The [`VideoDevice`] to add to the system.
    ///
    /// # Errors
    ///
    /// Returns a [`HwSpecError::DuplicateVideoDevice`] if the exact same device is already present.
    pub fn add_video_device(&mut self, device: VideoDevice) -> Result<(), HwSpecError> {
        if self.video.contains(&device) {
            return Err(HwSpecError::DuplicateVideoDevice);
        }
        self.video.push(device);
        Ok(())
    }

    pub fn video(&self) -> &[VideoDevice] {
        &self.video
    }

    pub fn set_cpu(&mut self, cpu: &str) -> Result<(), HwSpecError> {
        self.cpu = Cpu::from_str(cpu)?;
        Ok(())
    }

    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, HwSpecError> {
        let settings = Config::builder()
            .add_source(File::from(path.as_ref()).format(FileFormat::Toml))
            .build()
            .map_err(HwSpecError::ConfigBuild)?;

        settings
            .try_deserialize::<HwSpec>()
            .map_err(HwSpecError::Deserialize)
    }
    /// Sets the amount of system RAM.
    ///
    /// The `ram` parameter must be a human-readable string representing a memory size,
    /// such as `"640 KB"`, `"2 MB"`, or `"16 MiB"`. Both SI (e.g., KB, MB) and binary (e.g., KiB, MiB) units
    /// are supported. Unit case is ignored.
    ///
    /// # Arguments
    ///
    /// * `ram` - A string slice containing the desired RAM size and unit.
    ///
    /// # Errors
    ///
    /// Returns a [`HwSpecError::InvalidRamString`] if the string cannot be parsed.
    ///
    /// Returns a [`HwSpecError::TooMuchRamSpecified`] if the parsed RAM size cannot fit into a `u32`
    /// (i.e., exceeds 4 GiB). This coincides with the theoretical maximum of the 32-bit Intel platform.
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

pub fn deserialize_ram<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?; // <-- use String here
    const IGNORE_CASE: bool = true;

    let byte = Byte::parse_str(&s, IGNORE_CASE).map_err(serde::de::Error::custom)?;

    byte.try_into()
        .map_err(|_| serde::de::Error::custom("RAM size too large for x86 system."))
}

impl fmt::Display for HwSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DOSContainer hardware specification")?;
        writeln!(f, "-----------------------------------")?;
        writeln!(f, " CPU   : {}", self.cpu())?;
        writeln!(f, " RAM   : {} bytes", self.ram())?;

        let video_str = self
            .video()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(f, " Video : {}", video_str)?;

        let audio_str = self
            .audio()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(f, " Audio : {}", audio_str)?;

        Ok(())
    }
}

