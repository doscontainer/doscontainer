use error::DiskError;
use sectorsize::SectorSize;

mod error;
mod raw;
mod sectorsize;

pub trait BlockDevice {
    fn read_block(&self, block_number: u64) -> Result<Vec<u8>, DiskError>;
    fn write_block(&mut self, block_number: u64, data: &[u8]) -> Result<(), DiskError>;
    fn block_size(&self) -> usize;
    fn block_count(&self) -> u64;
}

pub trait Disk {
    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), DiskError>;
    fn write_sector(&mut self, lba: u64, buf: &[u8]) -> Result<(), DiskError>;
    fn sector_count(&self) -> u64;
    fn sector_size(&self) -> SectorSize;
}
