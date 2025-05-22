
use error::PlanError;
use hwspec::HwSpec;
use manifest::Manifest;

mod error;

pub struct InstallationPlanner {

}

impl InstallationPlanner {
    pub fn new(hwspec: &HwSpec, mut manifest: Manifest) -> Result<(), PlanError> {
        println!("{}", hwspec);
        println!("{}", manifest);
        
        
        for layer in manifest.mut_layers() {
            layer.1.download().unwrap();
        }
        Ok(())
    }
}
