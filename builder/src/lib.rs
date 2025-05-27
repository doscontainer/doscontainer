use std::path::Path;

use disk::{raw::RawImage, sectorsize::SectorSize, volume::Volume, Disk};
use filesystem::{fat12::Fat12, FileSystem};
use operatingsystem::vendor::OsVendor;
use planner::InstallationPlanner;
use specs::types::storage::FloppyType;

#[derive(Debug)]
pub struct Builder {
    planner: InstallationPlanner,
}

impl Builder {
    pub fn new(planner: InstallationPlanner) -> Self {
        Builder { planner }
    }

    pub fn build(&self, path: &Path) {
        let floppy_type = self
            .planner
            .hwspec()
            .floppy_type()
            .unwrap_or(FloppyType::F525_160);
        let os = self.planner.os();
        let mut disk = RawImage::new(path, SectorSize::S512, floppy_type.sector_count()).unwrap();
        // Are we running on IBM?
        if os.vendor() == OsVendor::IBM {
            disk.ibmwipe().unwrap();
        }
        // Write the OS Boot sector
        disk.write_sector(0, os.bootsector()).unwrap();
        let mut volume = Volume::new(&mut disk, 0, 320);
        let mut fat = Fat12::new(
            SectorSize::S512,
            1,
            313,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ),
        )
        .unwrap();
        fat.mksysfile("IBMBIO.COM", os.iosys_bytes(), None).unwrap();
        fat.mksysfile("IBMDOS.COM", os.msdossys_bytes(), None)
            .unwrap();
        fat.mkfile("COMMAND.COM", os.commandcom_bytes(), None)
            .unwrap();
        fat.write_fat();
        fat.write_rootdir();

        println!("{:?}", disk);
    }
}
