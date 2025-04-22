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

pub struct HddGeometry {
    cylinders: usize,
    heads: usize,
    sectors: usize,
}
