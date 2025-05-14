use std::{fmt, str::FromStr};

use serde::Deserialize;
use serde::de::{self, Deserializer};

use crate::error::HwSpecError;

#[derive(Debug, Deserialize)]
pub struct Floppy {
    #[serde(deserialize_with = "deserialize_floppy_type")]
    floppy_type: FloppyType,
}

impl Floppy {
    pub fn floppy_type(&self) -> FloppyType {
        self.floppy_type
    }
}

#[derive(Copy, Clone, Debug, Deserialize)]
pub enum FloppyType {
    F525_160,
    F525_320,
    F525_180,
    F525_360,
    F525_1200,
    F35_720,
    F35_1440,
    F35_2880,
}

impl FromStr for Floppy {
    type Err = HwSpecError;

    fn from_str(input: &str) -> Result<Self, HwSpecError> {
        let floppy_type = FloppyType::from_str(input)?;
        Ok(Self { floppy_type })
    }
}

impl FromStr for FloppyType {
    type Err = HwSpecError;

    fn from_str(input: &str) -> Result<Self, HwSpecError> {
        match input.to_uppercase().as_str() {
            "F525_160" | "F525160" | "160" | "160K" | "160KB" => Ok(Self::F525_160),
            "F525_180" | "F525180" | "180" | "180K" | "180KB" => Ok(Self::F525_180),
            "F525_320" | "F525320" | "320" | "320K" | "320KB" => Ok(Self::F525_320),
            "F525_360" | "F525360" | "360" | "360K" | "360KB" => Ok(Self::F525_360),
            "F525_1200" | "F525_12M" | "F5251200" | "1200" | "1200K" | "1200KB" | "1.2M"
            | "1.2MB" => Ok(Self::F525_1200),
            "F35_720" | "F35720" | "720" | "720K" | "720KB" => Ok(Self::F35_720),
            "F35_1440" | "F351440" | "F35_144" | "F35144" | "1440" | "1440K" | "1440KB"
            | "1.44M" | "1.44MB" => Ok(Self::F35_1440),
            "F35_2880" | "F352880" | "F35_288" | "F35288" | "2880" | "2880K" | "2880KB"
            | "2.88M" | "2.88MB" => Ok(Self::F35_2880),
            _ => Err(HwSpecError::InvalidFloppyType),
        }
    }
}

fn deserialize_floppy_type<'de, D>(deserializer: D) -> Result<FloppyType, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;
    FloppyType::from_str(&s).map_err(de::Error::custom)
}

impl fmt::Display for FloppyType {
    /// Provides a user-friendly string representation of a floppy disk type.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::F525_160 => "5.25\" single-sided 160KB",
            Self::F525_320 => "5.25\" double-sided 320KB",
            Self::F525_180 => "5.25\" single-sided 180KB",
            Self::F525_360 => "5.25\" double-sided 360KB",
            Self::F525_1200 => "5.25\" double-sided 1.2MB",
            Self::F35_720 => "3.5\" double density 720KB",
            Self::F35_1440 => "3.5\" high density 1.44MB",
            Self::F35_2880 => "3.5\" extended density 2.88MB",
        };
        write!(f, "{}", label)
    }
}

impl fmt::Display for Floppy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.floppy_type)
    }
}
