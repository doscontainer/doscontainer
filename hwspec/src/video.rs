use crate::error::HwSpecError;
use std::str::FromStr;

pub enum VideoDevice {
    HCG,
    CGA,
    EGA,
    MCGA,
    VGA,
    SVGA,
    XGA,
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
        match input.to_uppercase().as_str() {
            "HCG" => Ok(VideoDevice::HCG),
            "CGA" => Ok(VideoDevice::CGA),
            "EGA" => Ok(VideoDevice::EGA),
            "MCGA" => Ok(VideoDevice::MCGA),
            "VGA" => Ok(VideoDevice::VGA),
            "SVGA" => Ok(VideoDevice::SVGA),
            "XGA" => Ok(VideoDevice::SVGA),
            _ => Err(HwSpecError::InvalidVideoDevice),
        }
    }
}
