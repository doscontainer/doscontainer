use std::fmt;
use std::path::Path;
use std::str::FromStr;

use byte_unit::Byte;
use config::Config;
use config::File;
use config::FileFormat;
use cpu::Cpu;
use serde::{Deserialize, Deserializer};
use serde_with::serde_as;
use serde_with::OneOrMany;
use storage::Floppy;
use storage::FloppyType;
use video::VideoDevice;

use crate::error::SpecError;
use crate::types::audio::AudioDevice;
use crate::types::audio::AudioDeviceType;

pub mod cpu;
pub mod storage;
mod tests;
pub mod video;

/// Represents the hardware configuration of an MS-DOS compatible PC system.
#[serde_as]
#[derive(Debug, Deserialize)]
pub struct HwSpec {
    cpu: Cpu,
    #[serde(deserialize_with = "deserialize_ram")]
    ram: u32,
    #[serde(default)]
    audio: Vec<AudioDevice>,
    #[serde_as(as = "OneOrMany<_>")]
    video: Vec<VideoDevice>,
    floppy: Option<Floppy>,
}

impl Default for HwSpec {
    fn default() -> Self {
        HwSpec {
            cpu: Cpu::from_str("8088").unwrap(),
            ram: 0,
            audio: Vec::new(),
            video: Vec::new(),
            floppy: None,
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
    /// Returns a [`SpecError::DuplicateAudioDevice`] if the exact same device is already present.
    ///
    pub fn add_audio_device(&mut self, device: AudioDevice) -> Result<(), SpecError> {
        if self.audio.contains(&device) {
            return Err(SpecError::DuplicateAudioDevice);
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
    /// Returns a [`SpecError::DuplicateVideoDevice`] if the exact same device is already present.
    pub fn add_video_device(&mut self, device: VideoDevice) -> Result<(), SpecError> {
        if self.video.contains(&device) {
            return Err(SpecError::DuplicateVideoDevice);
        }
        self.video.push(device);
        Ok(())
    }

    pub fn video(&self) -> &[VideoDevice] {
        &self.video
    }

    pub fn set_cpu(&mut self, cpu: &str) -> Result<(), SpecError> {
        self.cpu = Cpu::from_str(cpu)?;
        Ok(())
    }

    /// Loads a `HwSpec` from a TOML file at the specified path.
    ///
    /// This function attempts to read a TOML file and deserialize its contents
    /// into a `HwSpec` instance. It uses the `config` crate to handle parsing
    /// and supports proper error mapping for build and deserialization issues.
    ///
    /// # Type Parameters
    /// - `P`: A type that can be referenced as a `Path`, such as `&str` or `PathBuf`.
    ///
    /// # Arguments
    /// - `path`: The path to the TOML file to load.
    ///
    /// # Returns
    /// - `Ok(HwSpec)`: If the file was successfully read and deserialized.
    /// - `Err(SpecError)`: If there was an error reading or deserializing the file.
    ///
    /// # Errors
    /// - Returns `SpecError::ConfigBuild` if the configuration builder fails.
    /// - Returns `SpecError::Deserialize` if deserialization into `Manifest` fails.
    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, SpecError> {
        let settings = Config::builder()
            .add_source(File::from(path.as_ref()).format(FileFormat::Toml))
            .build()
            .map_err(SpecError::ConfigBuild)?;

        settings
            .try_deserialize::<HwSpec>()
            .map_err(SpecError::Deserialize)
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
    /// Returns a [`SpecError::InvalidRamString`] if the string cannot be parsed.
    ///
    /// Returns a [`SpecError::TooMuchRamSpecified`] if the parsed RAM size cannot fit into a `u32`
    /// (i.e., exceeds 4 GiB). This coincides with the theoretical maximum of the 32-bit Intel platform.
    pub fn set_ram(&mut self, ram: &str) -> Result<(), SpecError> {
        const IGNORE_CASE: bool = true;
        let amount =
            Byte::parse_str(ram, IGNORE_CASE).map_err(|_| SpecError::InvalidRamString)?;
        self.ram = amount
            .try_into()
            .map_err(|_| SpecError::TooMuchRamSpecified)?;
        Ok(())
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }

    pub fn ram(&self) -> u32 {
        self.ram
    }

    pub fn floppy_type(&self) -> Option<FloppyType> {
        if let Some(disk) = &self.floppy {
            Some(disk.floppy_type())
        } else {
            None
        }
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
        writeln!(f, " CPU    : {}", self.cpu())?;
        writeln!(f, " RAM    : {} bytes", self.ram())?;

        let video_str = self
            .video()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(f, " Video  : {}", video_str)?;

        let audio_str = self
            .audio()
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        writeln!(f, " Audio  : {}", audio_str)?;

        if let Some(myfloppy) = &self.floppy {
            writeln!(f, " Floppy : {}", myfloppy)?;
        }
        Ok(())
    }
}
