use error::DiskError;
use sectorsize::SectorSize;

pub mod error;
pub mod raw;
pub mod sectorsize;
pub mod volume;

pub trait Disk {
    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), DiskError>;
    fn write_sector(&mut self, lba: u64, buf: &[u8]) -> Result<(), DiskError>;
    fn ibmwipe(&mut self) -> Result<(), DiskError>;
    fn sector_count(&self) -> u64;
    fn sector_size(&self) -> SectorSize;
}
