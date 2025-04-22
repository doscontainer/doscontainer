use std::fmt;

pub struct StorageDevice {
    class: StorageClass,
    floppy_type: Option<FloppyType>,
    geometry: Option<HddGeometry>,
}

/// Type-safe determination of the class of storage device
pub enum StorageClass {
    /// Floppy disk
    Floppy,
    /// Harddrive
    Hdd,
}

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

impl fmt::Display for FloppyType {
    /// Provides a user-friendly string representation of a floppy disk type.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            FloppyType::F525_160 => "5.25\" single-sided 160KB",
            FloppyType::F525_320 => "5.25\" double-sided 320KB",
            FloppyType::F525_180 => "5.25\" single-sided 180KB",
            FloppyType::F525_360 => "5.25\" double-sided 360KB",
            FloppyType::F525_1200 => "5.25\" double-sided 1.2MB",
            FloppyType::F35_720 => "3.5\" double density 720KB",
            FloppyType::F35_1440 => "3.5\" high density 1.44MB",
            FloppyType::F35_2880 => "3.5\" extended density 2.88MB",
        };
        write!(f, "{}", label)
    }
}

pub struct HddGeometry {
    cylinders: usize,
    heads: usize,
    sectors: usize,
}
