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
            .mksysfile("IBMBIO.COM", 1920, Some(date))
            .unwrap();
        filesystem
            .mksysfile("IBMDOS.COM", 6400, Some(date))
            .unwrap();
        filesystem.mkfile("COMMAND.COM", 3231, Some(date)).unwrap();
        println!("{:?}", filesystem.allocation_table());
        let fatbytes = IbmDos100::serialize_fat12(&filesystem.allocation_table()).unwrap();
        let databytes = IbmDos100::serialize_directory(
            filesystem.pool(),
            filesystem.pool().root_entry().unwrap(),
        )
        .unwrap();
        filesystem.volume.write_sector(0, os.bootsector()).unwrap();
        filesystem.volume.write_sector(1, &fatbytes).unwrap();
        filesystem.volume.write_sector(2, &fatbytes).unwrap();
        for (i, chunk) in databytes.chunks(512).enumerate() {
            filesystem.volume.write_sector(3 + i as u64, chunk).unwrap();
        }

        let iosys_clusters = &filesystem
            .pool()
            .entry_by_path(Path::new("IBMBIO.COM"))
            .unwrap()
            .cluster_map();

        let iosys_bytes = os.iosys_bytes();
        let mut offset = 0;

        for cluster in iosys_clusters.to_vec() {
            let sector = cluster + 5;
            let mut buffer = [0u8; 512];
            if offset < iosys_bytes.len() {
                let end = usize::min(offset + 512, iosys_bytes.len());
                buffer[..(end - offset)].copy_from_slice(&iosys_bytes[offset..end]);
            }
            filesystem.volume.write_sector(sector as u64, &buffer).unwrap();
            offset += 512;
        }

        let msdossys_clusters = filesystem
            .pool()
            .entry_by_path(Path::new("IBMDOS.COM"))
            .unwrap()
            .cluster_map();
        let msdossys_bytes = os.msdoss_bytes();
        let mut offset = 0;

        for cluster in msdossys_clusters.to_vec() {
            let sector = cluster + 5;
            let mut buffer = [0u8; 512];
            if offset < msdossys_bytes.len() {
                let end = usize::min(offset + 512, msdossys_bytes.len());
                buffer[..(end - offset)].copy_from_slice(&msdossys_bytes[offset..end]);
            }
            filesystem.volume.write_sector(sector as u64, &buffer).unwrap();
            offset += 512;
        }

        let commandcom_clusters = filesystem
            .pool()
            .entry_by_path(Path::new("COMMAND.COM"))
            .unwrap()
            .cluster_map();
        let commandcom_bytes = os.commandcom_bytes();
        let mut offset = 0;

        for cluster in commandcom_clusters.to_vec() {
            let sector = cluster + 5;
            let mut buffer = [0u8; 512];
            if offset < commandcom_bytes.len() {
                let end = usize::min(offset + 512, commandcom_bytes.len());
                buffer[..(end - offset)].copy_from_slice(&commandcom_bytes[offset..end]);
            }
            filesystem.volume.write_sector(sector as u64, &buffer).unwrap();
            offset += 512;
        }

        let layers = manifest.mut_layers();
        for layer in layers {
            println!("Downloading {}", layer.0);
            println!("{:?}", layer.1.download());
        }
        Ok(())
    }
}
