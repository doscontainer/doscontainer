use std::path::Path;

use disk::{raw::RawImage, sectorsize::SectorSize, Disk};
use error::PlanError;
use operatingsystem::OperatingSystem;
use specs::{hwspec::HwSpec, manifest::Manifest};

mod error;
mod ossupport;

pub struct InstallationPlanner {

}

impl InstallationPlanner {
    pub fn new(hwspec: &HwSpec, mut manifest: Manifest) -> Result<(), PlanError> {
        println!("{}",hwspec);
        println!("{}", manifest);

        let mut disk = RawImage::new(Path::new("/home/bvdwiel/test.img"), SectorSize::S512, 320).expect("Failed to create disk");
        let os = OperatingSystem::from_vendor_version("IBM", "1.00").unwrap();
        disk.write_sector(0, os.bootsector()).unwrap();

        let layers = manifest.mut_layers();
        for layer in layers {
            println!("Downloading {}", layer.0);
            println!("{:?}", layer.1.download());
        }
        Ok(())
    }
}
