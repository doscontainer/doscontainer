use std::path::Path;

use chrono::{NaiveDate, NaiveDateTime};
use disk::{raw::RawImage, sectorsize::SectorSize, volume::Volume, Disk};
use error::PlanError;
use filesystem::{
    fat12::Fat12,
    serializer::{ibmdos100::IbmDos100, DirectorySerializer, Fat12Serializer},
    FileSystem,
};
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
    fn is_compatible(hwspec: &HwSpec, os: &OsSupport) -> bool {
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
        disk.ibmwipe().unwrap();
        let mut volume = Volume::new(&mut disk, 0, 320);
        let os = OperatingSystem::from_vendor_version("IBM", "1.00").unwrap();
        let mut filesystem = Fat12::new(SectorSize::S512, 1, 312, &mut volume).unwrap();
        let date = NaiveDate::from_ymd_opt(1981, 8, 4)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        filesystem
            .mksysfile("IBMBIO.COM", os.iosys_bytes(), Some(date))
            .unwrap();
        filesystem
            .mksysfile("IBMDOS.COM", os.msdoss_bytes(), Some(date))
            .unwrap();
        filesystem.mkfile("COMMAND.COM", os.commandcom_bytes(), Some(date)).unwrap();

        // Do massively ugly hard-coded crud here!
        filesystem.write_crud();


        let layers = manifest.mut_layers();
        for layer in layers {
            println!("Downloading {}", layer.0);
            println!("{:?}", layer.1.download());
        }
        Ok(())
    }
}
