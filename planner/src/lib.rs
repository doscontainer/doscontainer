use error::PlanError;
use hwspec::HwSpec;
use manifest::Manifest;

mod error;

pub struct InstallationPlanner {

}

impl InstallationPlanner {
    pub fn new(hwspec: &HwSpec, mut manifest: Manifest) -> Result<(), PlanError> {
        println!("{}",hwspec);
        println!("{}", manifest);

        let disk = RawDisk::new();

        let layers = manifest.layers_mut();
        for layer in layers {
            println!("Downloading {}", layer.0);
            println!("{:?}", layer.1.download());
        }
        Ok(())
    }
}
