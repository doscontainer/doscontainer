use std::path::Path;

use disk::{raw::RawImage, sectorsize::SectorSize, volume::Volume, Disk};
use error::BuildError;
use filesystem::{fat12::Fat12, FileSystem};
use operatingsystem::{vendor::OsVendor, OperatingSystem};
use planner::InstallationPlanner;
use specs::types::storage::FloppyType;

mod error;

#[derive(Debug)]
pub struct Builder {
    planner: InstallationPlanner,
}

impl Builder {
    pub fn new(planner: InstallationPlanner) -> Self {
        Builder { planner }
    }

    pub fn build(&self, path: &Path) -> Result<(), BuildError> {
        let floppy_type = self
            .planner
            .hwspec()
            .floppy_type()
            .unwrap_or(FloppyType::F525_160);
        let os = self.planner.os();

        let mut disk = self.prepare_disk(path, floppy_type.sector_count(), os.vendor())?;
        self.install_boot_sector(&mut disk, os.bootsector())?;

        let mut volume = Volume::new(&mut disk, 0, 320);
        let mut fat = self.create_filesystem(&mut volume)?;

        self.write_sysfiles(&mut fat, os)?;

        Ok(())
    }

    fn prepare_disk(
        &self,
        path: &Path,
        sector_count: u64,
        vendor: OsVendor,
    ) -> Result<RawImage, BuildError> {
        let mut disk =
            RawImage::new(path, SectorSize::S512, sector_count).map_err(BuildError::DiskIoError)?;

        if vendor == OsVendor::IBM {
            disk.ibmwipe().map_err(BuildError::DiskIoError)?;
        }

        Ok(disk)
    }

    fn install_boot_sector(
        &self,
        disk: &mut impl Disk,
        boot_sector: &[u8],
    ) -> Result<(), BuildError> {
        disk.write_sector(0, boot_sector)
            .map_err(BuildError::DiskIoError)
    }

    fn create_filesystem<'a, D: Disk>(&self, volume: &'a mut Volume<'a, D>) -> Result<Fat12<'a, D>, BuildError> {
        Fat12::new(
            SectorSize::S512,
            1,
            313,
            volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ),
        )
        .map_err(|_| BuildError::FileSystemError)
    }

    fn write_sysfiles<D: Disk>(
        &self,
        fat: &mut Fat12<D>,
        os: &OperatingSystem
    ) -> Result<(), BuildError> {
        fat.mksysfile("IBMBIO.COM", os.iosys_bytes(), None)
            .map_err(|_|BuildError::FileSystemError)?;
        fat.mksysfile("IBMDOS.COM", os.msdossys_bytes(), None)
            .map_err(|_|BuildError::FileSystemError)?;
        fat.mkfile("COMMAND.COM", os.commandcom_bytes(), None)
            .map_err(|_|BuildError::FileSystemError)?;

        fat.write_fat();
        fat.write_rootdir();

        Ok(())
    }
}
