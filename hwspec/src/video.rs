use serde::Deserialize;

use crate::error::HwSpecError;
use std::{fmt, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum VideoDevice {
    HCG,
    CGA,
    EGA,
    MCGA,
    VGA,
    SVGA,
    XGA,
}

impl fmt::Display for VideoDevice {
    /// Provides a user-friendly string representation of the CPU type.
    ///
    /// This implementation formats each CPU type into a human-readable string that represents
    /// the full name of the processor, e.g., "Intel 8086" or "Intel 80486DX".
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let name = match self {
            VideoDevice::HCG => "Hercules Monochrome",
            VideoDevice::CGA => "IBM CGA or compatible",
            VideoDevice::EGA => "IBM EGA or compatible",
            VideoDevice::VGA => "IBM VGA or compatible",
            VideoDevice::MCGA => "IBM MCGA or compatible",
            VideoDevice::SVGA => "SuperVGA (anything above regular VGA)",
            VideoDevice::XGA => "IBM XGA or compatible",
        };
        write!(f, "{}", name)
    }
}

impl<'de> Deserialize<'de> for VideoDevice {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(serde::de::Error::custom)
    }
}

impl FromStr for VideoDevice {
    type Err = HwSpecError;

    /// Converts a string into the corresponding `VideoDevice` variant.
    ///
    /// This method attempts to parse a string into a CPU variant. It supports the
    /// version of the device names like "CGA" or "EGA" in mixed case. If the input string
    /// does not match a valid video device name, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice representing a video device type like "CGA" or "ega".
    ///
    /// # Returns
    ///
    /// * `Ok(VideoDevice)` - The corresponding video device if the string matches a valid video device name.
    /// * `Err(HwStateError)` - An error if the string does not match any valid video device name.
    fn from_str(input: &str) -> Result<Self, HwSpecError> {
        match input.trim().to_lowercase().as_str() {
            "hcg" | "hercules" => Ok(VideoDevice::HCG),
            "cga" => Ok(VideoDevice::CGA),
            "ega" => Ok(VideoDevice::EGA),
            "mcga" => Ok(VideoDevice::MCGA),
            "vga" => Ok(VideoDevice::VGA),
            "svga" => Ok(VideoDevice::SVGA),
            "xga" => Ok(VideoDevice::XGA),
            _ => Err(HwSpecError::InvalidVideoDevice),
        }
    }
}
