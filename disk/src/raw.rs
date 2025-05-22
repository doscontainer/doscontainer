use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use crate::error::DiskError;
use crate::{sectorsize::SectorSize, Disk};

pub struct RawImage {
    file: File,
    sector_size: SectorSize,
    sector_count: u64,
}

impl RawImage {
    pub fn new(path: &Path, sector_size: SectorSize, sector_count: u64) -> Result<Self, DiskError> {
        // Try to create a new file, fail if it already exists
        let file = File::options()
            .read(true)
            .write(true)
            .create_new(true) 
            .open(path)
            .map_err(|_| DiskError::FileAlreadyExists)?;

        // Allocate the disk image file to the size we require
        file.set_len(sector_count * sector_size.as_u64())
            .map_err(|_| DiskError::IoError)?;

        Ok(Self {
            file,
            sector_size,
            sector_count,
        })
    }
}

impl Disk for RawImage {
    fn read_sector(&mut self, lba: u64, buf: &mut [u8]) -> Result<(), DiskError> {
        let sector_size = self.sector_size.as_usize();

        if buf.len() < sector_size {
            return Err(DiskError::BufferTooSmall);
        }

        if lba >= self.sector_count {
            return Err(DiskError::OutOfBounds);
        }

        let offset = lba * sector_size as u64;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|_| DiskError::SeekFailed)?;
        self.file
            .read_exact(&mut buf[..sector_size])
            .map_err(|_| DiskError::ReadFailed)?;

        Ok(())
    }

    fn write_sector(&mut self, lba: u64, buf: &[u8]) -> Result<(), DiskError> {
        let sector_size = self.sector_size.as_usize();

        if buf.len() < sector_size {
            return Err(DiskError::BufferTooSmall);
        }

        if lba >= self.sector_count {
            return Err(DiskError::OutOfBounds);
        }

        let offset = lba * sector_size as u64;
        self.file
            .seek(SeekFrom::Start(offset))
            .map_err(|_| DiskError::SeekFailed)?;
        self.file
            .write_all(&buf[..sector_size])
            .map_err(|_| DiskError::WriteFailed)?;
        self.file.flush().map_err(|_| DiskError::FlushFailed)?;

        Ok(())
    }

    fn sector_count(&self) -> u64 {
        self.sector_count
    }

    fn sector_size(&self) -> SectorSize {
        self.sector_size
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn raw_image_creation_succeeds() {
        let sector_size = SectorSize::S512;
        let sector_count = 4;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        let raw = RawImage::new(&path, sector_size, sector_count);
        assert!(raw.is_ok(), "Failed to create RawImage");

        let metadata = std::fs::metadata(&path).unwrap();
        assert_eq!(
            metadata.len(),
            sector_size.as_u64() * sector_count,
            "File size incorrect"
        );
    }

    #[test]
    fn raw_image_creation_fails_if_file_exists() {
        let sector_size = SectorSize::S512;
        let sector_count = 4;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        // Pre-create the file
        File::create(&path).unwrap();

        let raw = RawImage::new(&path, sector_size, sector_count);
        assert!(matches!(raw, Err(DiskError::FileAlreadyExists)));
    }

    #[test]
    fn write_and_read_sector_works() {
        let sector_size = SectorSize::S512;
        let sector_count = 2;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        let mut raw = RawImage::new(&path, sector_size, sector_count).unwrap();

        let write_data = [0xAA; 512];
        let mut read_buf = [0x00; 512];

        // Write to sector 1
        raw.write_sector(1, &write_data).unwrap();

        // Read it back
        raw.read_sector(1, &mut read_buf).unwrap();

        assert_eq!(write_data, read_buf, "Data mismatch");
    }

    #[test]
    fn read_sector_out_of_bounds_fails() {
        let sector_size = SectorSize::S512;
        let sector_count = 1;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        let mut raw = RawImage::new(&path, sector_size, sector_count).unwrap();

        let mut buf = [0x00; 512];
        let result = raw.read_sector(5, &mut buf);

        assert!(matches!(result, Err(DiskError::OutOfBounds)));
    }

    #[test]
    fn write_sector_buffer_too_small_fails() {
        let sector_size = SectorSize::S512;
        let sector_count = 1;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        let mut raw = RawImage::new(&path, sector_size, sector_count).unwrap();

        let buf = [0xAB; 100]; // Too small
        let result = raw.write_sector(0, &buf);

        assert!(matches!(result, Err(DiskError::BufferTooSmall)));
    }

    #[test]
    fn read_sector_buffer_too_small_fails() {
        let sector_size = SectorSize::S512;
        let sector_count = 1;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        let mut raw = RawImage::new(&path, sector_size, sector_count).unwrap();

        let mut buf = [0x00; 128]; // Too small
        let result = raw.read_sector(0, &mut buf);

        assert!(matches!(result, Err(DiskError::BufferTooSmall)));
    }

    #[test]
    fn sector_size_and_count_are_correct() {
        let sector_size = SectorSize::S512;
        let sector_count = 8;

        let tmpdir = tempdir().unwrap();
        let path = tmpdir.path().join("disk.img");

        let raw = RawImage::new(&path, sector_size, sector_count).unwrap();

        assert_eq!(raw.sector_size(), sector_size);
        assert_eq!(raw.sector_count(), sector_count);
    }
}
