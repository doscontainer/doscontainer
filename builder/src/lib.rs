use std::path::Path;

use disk::{raw::RawImage, sectorsize::SectorSize, Disk};
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

    pub fn build(&self) {
        let floppy_type = self.planner.hwspec().floppy_type().unwrap_or(FloppyType::F525_160);
        let os = self.planner.os();
        let mut disk = RawImage::new(Path::new("/home/bvdwiel/test.img"),SectorSize::S512, floppy_type.sector_count()).unwrap();
        // Are we running on IBM?
        if os.vendor() == OsVendor::IBM {
            disk.ibmwipe().unwrap();
        }
        // Write the OS Boot sector
        disk.write_sector(0, os.bootsector()).unwrap();
        println!("{:?}", disk);
    }
}
