use std::{fmt, str::FromStr};

use serde::Deserialize;

use crate::error::CommonError;

#[derive(Copy, Clone, Debug, Deserialize, PartialEq)]
pub enum Floppy {
    F525_160,
    F525_320,
    F525_180,
    F525_360,
    F525_1200,
    F35_720,
    F35_1440,
    F35_2880,
}

impl Floppy {
    pub fn sector_count(&self) -> u64 {
        match self {
            Floppy::F525_160 => 40 * 1 * 8,
            Floppy::F525_180 => 40 * 1 * 9,
            Floppy::F525_320 => 40 * 2 * 8,
            Floppy::F525_360 => 40 * 2 * 9,
            Floppy::F525_1200 => 80 * 2 * 15,
            Floppy::F35_720 => 80 * 2 * 9,
            Floppy::F35_1440 => 80 * 2 * 18,
            Floppy::F35_2880 => 80 * 2 * 36,
        }
    }

    /// This is only here because it may become dynamic
    /// if we ever support 8" floppies, which had 128KB
    /// sectors.
    pub fn sector_size(&self) -> u64 {
        512
    }
}

impl FromStr for Floppy {
    type Err = CommonError;

    fn from_str(input: &str) -> Result<Self, CommonError> {
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
            _ => Err(CommonError::InvalidFloppyType),
        }
    }
}

impl fmt::Display for Floppy {
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