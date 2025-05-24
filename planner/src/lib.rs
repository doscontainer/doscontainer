use std::path::Path;

use disk::{raw::RawImage, sectorsize::SectorSize, Disk};
use error::PlanError;
use operatingsystem::OperatingSystem;
use ossupport::OsSupport;
use specs::{hwspec::HwSpec, manifest::Manifest};

mod error;
mod ossupport;

pub struct InstallationPlanner {}

impl InstallationPlanner {
    /// Pass an HwSpec and an OsSupport into this function to figure out
    /// if the requested OS will run on the provided hardware. This function
    /// filters out operating systems so that only a compatible set remains.
    pub fn is_compatible(hwspec: &HwSpec, os: &OsSupport) -> bool {
        hwspec.ram() >= os.min_ram_kib
            && os.supported_cpu_families.contains(&hwspec.cpu().family())
            && hwspec
                .floppy_type()
                .as_ref()
                .map_or(false, |f| os.supported_floppies.contains(f))
            && !hwspec.video().is_empty()
            && hwspec
                .video()
                .iter()
                .any(|v| os.supported_video.contains(v))
    }

    pub fn new(hwspec: &HwSpec, mut manifest: Manifest) -> Result<(), PlanError> {
        println!("{}", hwspec);
        println!("{}", manifest);

        let mut disk = RawImage::new(Path::new("/home/bvdwiel/test.img"), SectorSize::S512, 320)
            .expect("Failed to create disk");
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
