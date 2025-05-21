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
    pub fn new(path: &Path, sector_size: SectorSize) -> Result<Self, DiskError> {
        // Open the file for reading and writing
        let file = File::options()
            .read(true)
            .write(true)
            .open(path)
            .map_err(|_| DiskError::FileOpenFailed)?;

        // Determine file size
        let metadata = file.metadata().map_err(|_| DiskError::FileMetadataFailed)?;
        let file_size = metadata.len();

        let sector_size_usize = sector_size.as_usize() as u64;

        if file_size % sector_size_usize != 0 {
            return Err(DiskError::InvalidFileSize);
        }

        let sector_count = file_size / sector_size_usize;

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
    use tempfile::NamedTempFile;

    fn create_temp_file_with_size(size: usize) -> NamedTempFile {
        let mut tmp = NamedTempFile::new().expect("Failed to create temp file");
        tmp.as_file_mut()
            .set_len(size as u64)
            .expect("Failed to set file size");
        tmp
    }

     #[test]
    fn raw_image_new_works_and_cleans_up() {
        let sector_size = SectorSize::S512;
        let sector_count = 4;
        let total_size = sector_size.as_usize() * sector_count;

        // Create a temporary file
        let mut tmpfile = NamedTempFile::new().expect("Failed to create temp file");
        tmpfile
            .as_file_mut()
            .set_len(total_size as u64)
            .expect("Failed to set size");

        let path = tmpfile.path();

        // Construct RawImage
        let raw_image = RawImage::new(path, sector_size);
        assert!(raw_image.is_ok());
    }

    #[test]
    fn raw_image_new_with_valid_file() {
        let sector_size = SectorSize::S512;
        let file_size = sector_size.as_usize() * 4;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let raw_image = RawImage::new(path, sector_size);
        assert!(raw_image.is_ok());
        let raw_image = raw_image.unwrap();
        assert_eq!(raw_image.sector_count(), 4);
        assert_eq!(raw_image.sector_size(), sector_size);
    }

    #[test]
    fn raw_image_new_fails_with_invalid_file_size() {
        let sector_size = SectorSize::S512;
        // File size not multiple of sector size
        let file_size = (sector_size.as_usize() * 4) + 1;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let raw_image = RawImage::new(path, sector_size);
        assert!(matches!(raw_image, Err(DiskError::InvalidFileSize)));
    }

    #[test]
    fn read_and_write_sector_success() {
        let sector_size = SectorSize::S512;
        let file_size = sector_size.as_usize() * 2;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let mut raw_image = RawImage::new(path, sector_size).expect("Failed to create RawImage");

        // Prepare a buffer with data to write
        let write_buf = vec![0xAB; sector_size.as_usize()];
        raw_image
            .write_sector(1, &write_buf)
            .expect("Failed to write sector");

        // Prepare a buffer to read into
        let mut read_buf = vec![0u8; sector_size.as_usize()];
        raw_image
            .read_sector(1, &mut read_buf)
            .expect("Failed to read sector");

        assert_eq!(write_buf, read_buf);
    }

    #[test]
    fn read_sector_fails_with_buffer_too_small() {
        let sector_size = SectorSize::S512;
        let file_size = sector_size.as_usize() * 1;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let mut raw_image = RawImage::new(path, sector_size).expect("Failed to create RawImage");
        let mut small_buf = vec![0u8; sector_size.as_usize() - 1];

        let result = raw_image.read_sector(0, &mut small_buf);
        assert!(matches!(result, Err(DiskError::BufferTooSmall)));
    }

    #[test]
    fn write_sector_fails_with_buffer_too_small() {
        let sector_size = SectorSize::S512;
        let file_size = sector_size.as_usize() * 1;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let mut raw_image = RawImage::new(path, sector_size).expect("Failed to create RawImage");
        let small_buf = vec![0u8; sector_size.as_usize() - 1];

        let result = raw_image.write_sector(0, &small_buf);
        assert!(matches!(result, Err(DiskError::BufferTooSmall)));
    }

    #[test]
    fn read_sector_fails_out_of_bounds() {
        let sector_size = SectorSize::S512;
        let file_size = sector_size.as_usize() * 1;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let mut raw_image = RawImage::new(path, sector_size).expect("Failed to create RawImage");
        let mut buf = vec![0u8; sector_size.as_usize()];

        let result = raw_image.read_sector(1, &mut buf);
        assert!(matches!(result, Err(DiskError::OutOfBounds)));
    }

    #[test]
    fn write_sector_fails_out_of_bounds() {
        let sector_size = SectorSize::S512;
        let file_size = sector_size.as_usize() * 1;
        let tmp = create_temp_file_with_size(file_size);
        let path = tmp.path();

        let mut raw_image = RawImage::new(path, sector_size).expect("Failed to create RawImage");
        let buf = vec![0u8; sector_size.as_usize()];

        let result = raw_image.write_sector(1, &buf);
        assert!(matches!(result, Err(DiskError::OutOfBounds)));
    }
}
