use std::{fmt, str::FromStr};

use crate::error::HwSpecError;

pub struct StorageDevice {
    class: StorageClass,
    floppy_type: Option<FloppyType>,
    geometry: Option<HddGeometry>,
}

impl StorageDevice {
    /// Constructor for use when you already know you want a floppy. Pass in a string slice
    /// and you'll get a complete StorageDevice in return.
    pub fn new_floppy(floppy_type: &str) -> Result<Self, HwSpecError> {
        Ok(StorageDevice {
            class: StorageClass::Floppy,
            floppy_type: Some(FloppyType::from_str(floppy_type)?),
            geometry: None,
        })
    }

    pub fn new_harddisk(
        cylinders: usize,
        heads: usize,
        sectors: usize,
    ) -> Result<Self, HwSpecError> {
        Ok(StorageDevice {
            class: StorageClass::Hdd,
            floppy_type: None,
            geometry: Some(HddGeometry::new(cylinders, heads, sectors)?),
        })
    }

    pub fn class(&self) -> StorageClass {
        self.class
    }

    pub fn floppy_type(&self) -> Option<FloppyType> {
        self.floppy_type
    }

    pub fn geometry(&self) -> Option<&HddGeometry> {
        self.geometry.as_ref()
    }
}

/// Type-safe determination of the class of storage device
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum StorageClass {
    /// Floppy disk
    Floppy,
    /// Harddrive
    Hdd,
}

#[derive(Copy, Clone)]
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

impl FromStr for StorageClass {
    type Err = HwSpecError;

    fn from_str(input: &str) -> Result<Self, HwSpecError> {
        match input.to_uppercase().as_str() {
            "FLOPPY" | "FDD" | "FLOPPYDRIVE" | "FLOPPYDISK" | "FLOPPY DISK" | "FLOPPY DRIVE" => {
                Ok(Self::Floppy)
            }
            "HDD" | "HARDDISK" | "HARDDRIVE" | "HARD DISK" | "HARD DRIVE" => Ok(Self::Hdd),
            _ => Err(HwSpecError::InvalidStorageClass),
        }
    }
}

impl fmt::Display for StorageClass {
    /// Provides a user-friendly string representation of a floppy disk type.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            Self::Floppy => "Floppy drive",
            Self::Hdd => "Hard drive",
        };
        write!(f, "{}", label)
    }
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

pub struct HddGeometry {
    cylinders: usize,
    heads: usize,
    sectors: usize,
}

impl fmt::Display for HddGeometry {
    /// Provides a user-friendly string representation of an HDD geometry.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cylinders: {}, Heads: {}, Sectors per track: {}.",
            self.cylinders, self.heads, self.sectors
        )
    }
}

impl HddGeometry {
    pub fn new(cylinders: usize, heads: usize, sectors: usize) -> Result<Self, HwSpecError> {
        if cylinders == 0 || heads == 0 || sectors == 0 {
            return Err(HwSpecError::ValueMayNotBeZero);
        }
        if cylinders > 1024 {
            return Err(HwSpecError::TooManyCylinders);
        }
        if heads > 16 {
            return Err(HwSpecError::TooManyHeads);
        }
        if sectors > 63 {
            return Err(HwSpecError::TooManySectors);
        }
        Ok(HddGeometry {
            cylinders,
            heads,
            sectors,
        })
    }
}
