use std::path::Path;

use disk::{raw::RawImage, sectorsize::SectorSize, volume::Volume, Disk};
use error::BuildError;
use filesystem::{fat12::Fat12, FileSystem};
use operatingsystem::{vendor::OsVendor, OperatingSystem};
use planner::InstallationPlanner;

mod error;

#[derive(Debug)]
pub struct Builder {
    planner: InstallationPlanner,
}

impl Builder {
    pub fn new(planner: InstallationPlanner) -> Self {
        Builder { planner }
    }

    pub fn build(&mut self, path: &Path) -> Result<(), BuildError> {
        let os = self.planner.os();

        let (mut disk, sector_count) = {
            if let Some(floppy_type) = self.planner.hwspec().floppy_type() {
                let sector_count = floppy_type.sector_count();
                let disk = RawImage::new_floppy(path, floppy_type)?;
                (disk, sector_count)
            } else {
                return Err(BuildError::CanBuildOnlyFloppiesForNow);
            }
        };

        // Do the IBM thing if we're dealing with their equipment
        if os.vendor() == OsVendor::IBM {
            disk.ibmwipe()?;
        }

        self.install_boot_sector(&mut disk, os.bootsector())?;

        let mut volume = Volume::new(&mut disk, 0, sector_count);
        let mut fat = self.create_filesystem(&mut volume)?;

        Ok(())
    }

    fn install_boot_sector(
        &self,
        disk: &mut impl Disk,
        boot_sector: &[u8],
    ) -> Result<(), BuildError> {
        disk.write_sector(0, boot_sector)
            .map_err(BuildError::DiskIoError)
    }

    fn create_filesystem<'a, D: Disk>(
        &self,
        volume: &'a mut Volume<'a, D>,
    ) -> Result<Fat12<'a, D>, BuildError> {
        Fat12::new(
            SectorSize::S512,
            1,
            313,
            volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ),
            None,
        )
        .map_err(|_| BuildError::FileSystemError)
    }

    fn write_sysfiles<D: Disk>(
        &self,
        fat: &mut Fat12<D>,
        os: &OperatingSystem,
    ) -> Result<(), BuildError> {
        fat.mksysfile(os.iosys(), os.iosys_bytes(), None)
            .map_err(|_| BuildError::FileSystemError)?;
        fat.mksysfile(os.msdossys(), os.msdossys_bytes(), None)
            .map_err(|_| BuildError::FileSystemError)?;
        fat.mkfile("COMMAND.COM", os.commandcom_bytes(), None)
            .map_err(|_| BuildError::FileSystemError)?;

        fat.write_fat();
        fat.write_rootdir();

        Ok(())
    }
}
