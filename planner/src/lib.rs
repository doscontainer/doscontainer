use disk::disktype::DiskType;
use error::PlanError;
use hwspec::HwSpec;
use hwspec::storage::FloppyType;
use manifest::Manifest;

mod error;

pub struct InstallationPlanner {
    steps: Vec<InstallationStep>,
}
pub struct InstallationStep {}

impl InstallationPlanner {
    pub fn new(hwspec: &HwSpec, manifest: &Manifest) -> Result<Self, PlanError> {
        Ok(InstallationPlanner { steps: Vec::new() })
    }

    /// Determine what kind of Disk to use for our build. Compares what's in the
    /// HwSpec versus what the Manifest needs.
    fn determine_disk(hwspec: &HwSpec, manifest: &Manifest) -> Option<DiskType> {
        match hwspec.floppy_type() {
            Some(FloppyType::F525_160) => Some(DiskType::F525_160),
            Some(FloppyType::F525_320) => Some(DiskType::F525_320),
            Some(FloppyType::F525_180) => Some(DiskType::F525_180),
            Some(FloppyType::F525_360) => Some(DiskType::F525_360),
            Some(FloppyType::F525_1200) => Some(DiskType::F525_1200),
            Some(FloppyType::F35_720) => Some(DiskType::F35_720),
            Some(FloppyType::F35_1440) => Some(DiskType::F35_1440),
            Some(FloppyType::F35_2880) => Some(DiskType::F35_2880),
            None => None
        }
    }
}
