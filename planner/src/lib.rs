
use error::PlanError;
use hwspec::storage::FloppyType;
use hwspec::HwSpec;
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
}
