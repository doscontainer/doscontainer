use crate::{sectorsize::SectorSize, Disk};

pub struct RawImage {
    file: std::fs::File,
}

impl Disk for RawImage {
    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), crate::error::DiskError> {
        todo!()
    }

    fn write_sector(&mut self, lba: u64, buf: &[u8]) -> Result<(), crate::error::DiskError> {
        todo!()
    }

    fn sector_count(&self) -> u64 {
        todo!()
    }

    fn sector_size(&self) -> SectorSize {
        todo!()
    }
}