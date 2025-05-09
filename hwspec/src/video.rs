use serde::Deserialize;

use crate::error::HwSpecError;
use std::str::FromStr;

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
