use crate::{error::DiskError, Disk};

pub struct Volume<D: Disk> {
    disk: D,
    start_sector: u64,
    sector_count: u64,
}

impl<D: Disk> Volume<D> {
    pub fn new(disk: D, start_sector: u64, sector_count: u64) -> Self {
        Self {
            disk,
            start_sector,
            sector_count,
        }
    }

    pub fn read_sector(&mut self, sector: u64, buf: &mut [u8]) -> Result<(), DiskError> {
        if sector >= self.sector_count {
            return Err(DiskError::OutOfBounds);
        }
        self.disk.read_sector(self.start_sector + sector, buf)
    }

    pub fn write_sector(&mut self, sector: u64, buf: &[u8]) -> Result<(), DiskError> {
        if sector >= self.sector_count {
            return Err(DiskError::OutOfBounds);
        }
        self.disk.write_sector(self.start_sector + sector, buf)
    }

    pub fn start_sector(&self) -> u64 {
        self.start_sector
    }

    pub fn sector_count(&self) -> u64 {
        self.sector_count
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{error::DiskError, Disk};

    // Mock Disk that uses an in-memory Vec<u8> as storage
    struct MockDisk {
        data: Vec<u8>,
        sector_size: usize,
        sector_count: u64,
    }

    impl MockDisk {
        fn new(sector_count: u64, sector_size: usize) -> Self {
            Self {
                data: vec![0; sector_count as usize * sector_size],
                sector_size,
                sector_count,
            }
        }
    }

    impl Disk for MockDisk {
        fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), DiskError> {
            let offset = lba as usize * self.sector_size;
            if lba >= self.sector_count || buf.len() < self.sector_size {
                return Err(DiskError::OutOfBounds);
            }
            buf[..self.sector_size].copy_from_slice(&self.data[offset..offset + self.sector_size]);
            Ok(())
        }

        fn write_sector(&mut self, lba: u64, buf: &[u8]) -> Result<(), DiskError> {
            let offset = lba as usize * self.sector_size;
            if lba >= self.sector_count || buf.len() < self.sector_size {
                return Err(DiskError::OutOfBounds);
            }
            self.data[offset..offset + self.sector_size].copy_from_slice(&buf[..self.sector_size]);
            Ok(())
        }

        fn sector_count(&self) -> u64 {
            self.sector_count
        }

        fn sector_size(&self) -> crate::SectorSize {
            crate::sectorsize::SectorSize::try_from(self.sector_size).unwrap()
        }
    }

    #[test]
    fn test_volume_read_write() {
        let sector_size = 512;
        let disk_sectors = 10;
        let mock_disk = MockDisk::new(disk_sectors, sector_size);

        let mut volume = Volume::new(mock_disk, 2, 5);

        // Write to sector 0 of the volume (which maps to sector 2 on the disk)
        let write_data = vec![0xAB; sector_size];
        volume.write_sector(0, &write_data).unwrap();

        // Read it back
        let mut read_buf = vec![0u8; sector_size];
        volume.read_sector(0, &mut read_buf).unwrap();

        assert_eq!(read_buf, write_data);

        // Reading or writing outside volume bounds should fail
        assert!(volume.read_sector(5, &mut read_buf).is_err());
        assert!(volume.write_sector(5, &write_data).is_err());
    }

    #[test]
    fn test_volume_out_of_bounds() {
        let sector_size = 512;
        let disk_sectors = 10;
        let mock_disk = MockDisk::new(disk_sectors, sector_size);

        let mut volume = Volume::new(mock_disk, 8, 2);

        let mut buf = vec![0u8; sector_size];
        assert!(volume.read_sector(2, &mut buf).is_err()); // volume sector 2 is out of bounds (max is 1)
        assert!(volume.write_sector(2, &buf).is_err());
    }
}
