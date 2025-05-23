use disk::disktype::DiskType;
use error::PlanError;
use specs::{hwspec::HwSpec, manifest::Manifest};
mod error;

pub struct InstallationPlanner {
}

impl InstallationPlanner {
    pub fn new(hwspec: &HwSpec, manifest: Manifest) -> Result<(), PlanError> {
        println!("{}",hwspec);
        println!("{}", manifest);
        Ok(())
    }

}
