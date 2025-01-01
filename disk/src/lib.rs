pub mod chs;
pub mod disktype;
pub mod error;
pub mod floppy;
pub mod geometry;
mod sector;
pub mod volume;

use std::{
    fs::File,
    io::{Read, Seek, SeekFrom, Write},
};

use chs::CHS;
use disktype::DiskType;
use volume::Volume;

use crate::{error::DiskError, geometry::Geometry, sector::Sector};

pub trait Disk {
    fn geometry(&self) -> Result<Geometry, DiskError>;
    fn sector_count(&self) -> Result<usize, DiskError>;
    fn sector_size(&self) -> Result<usize, DiskError>;
    fn file(&self) -> &File;
    fn disktype(&self) -> DiskType;
    fn volumes(&self) -> &Vec<Volume>;
    fn volumes_mut(&mut self) -> &mut Vec<Volume>;

    fn add_volume(&mut self, start_sector: usize, end_sector: usize) -> Result<(), DiskError> {
        match self.disktype() {
            // Floppies have fixed, well-known sizes and will only ever have one volume
            // that spans the entire usable storage area of the disk. We ignore the input
            // values for start_sector and end_sector for these types of disk.
            DiskType::F35_1440
            | DiskType::F35_2880
            | DiskType::F35_720
            | DiskType::F525_1200
            | DiskType::F525_160
            | DiskType::F525_180
            | DiskType::F525_320
            | DiskType::F525_360 => {
                // Make sure we don't have any existing volumes yet.
                if !self.volumes().is_empty() {
                    return Err(DiskError::VolumeAlreadyExists);
                }

                // Create a volume that spans the entire medium
                if let Some(disk_size) = self.disktype().sector_count() {
                    let volume = Volume::new(0, disk_size)?;
                    self.volumes_mut().push(volume);
                } else {
                    return Err(DiskError::InvalidVolumeSize);
                }
            }
            // Hard disks support from 1 to 4 volumes (for now) and come in many sizes
            DiskType::HardDisk => {
                // Hard disks are not supported yet, but code should go here eventually.
                todo!()
            }
        }
        Ok(())
    }

    /// Wipes the disk for use with IBM hardware by filling the specified portion of the disk with `0xF6` byte values.
    ///
    /// Historically, IBM operating systems would fill the data area of any disk it formats with `0xF6` bytes,
    /// which is not always the behavior of other versions of DOS. This function emulates that specific IBM behavior,
    /// ensuring compatibility with IBM OS'es.
    ///
    /// This function starts wiping the disk from the given `sector_offset` and fills the remaining sectors
    /// with `0xF6` bytes, overwriting any existing data.
    ///
    /// # Parameters
    ///
    /// - `sector_offset`: The starting sector from which the wipe will begin. This sector and all subsequent
    ///   sectors will be filled with `0xF6`.
    ///
    /// # Returns
    ///
    /// - `Ok(())`: If the wipe operation completes successfully.
    /// - `Err(DiskError)`: If the `sector_offset` is out of range, or if an error occurs while writing to the disk.
    ///
    /// # Errors
    ///
    /// - `DiskError::SectorOutOfRange`: If the provided `sector_offset` exceeds the number of sectors on the disk.
    /// - Other errors related to disk I/O can also be propagated via `DiskError`.
    fn ibm_wipe(&mut self, sector_offset: usize) -> Result<(), DiskError> {
        if sector_offset > self.sector_count()? {
            return Err(DiskError::SectorOutOfRange);
        }
        let data: [u8; 512] = [0xF6; 512];
        for sector in sector_offset..self.sector_count()? {
            self.write_lba(sector.try_into().unwrap(), &data)?;
        }
        Ok(())
    }

    /// Write a sector to a CHS address
    fn write_chs(&mut self, address: &CHS, data: &[u8]) -> Result<(), DiskError> {
        // Convert to LBA
        let sector_lba = address.to_lba(&self.geometry()?)?;
        // Use the lba-writer to perform the action
        self.write_lba(sector_lba, data)?;
        Ok(())
    }

    /// Read a sector from a CHS address
    fn read_chs(&mut self, address: &CHS) -> Result<Sector, DiskError> {
        // Convert to LBA
        let sector_lba = address.to_lba(&self.geometry()?)?;
        // Use the lba-reader to perform the action
        self.read_lba(sector_lba)
    }

    /// Write a sector to an LBA address (index) inside the Disk.
    fn write_lba(&mut self, index: u32, data: &[u8]) -> Result<(), DiskError> {
        let padded_data = pad_to_nearest(data)?;
        self.file()
            .seek(SeekFrom::Start(self.sector_size()? as u64 * index as u64))?;
        self.file().write_all(&padded_data)?;
        Ok(())
    }

    /// Read a sector from an LBA address (index) inside the Disk.
    fn read_lba(&self, index: u32) -> Result<Sector, DiskError> {
        // Bounds check: sector must exist.
        if index as usize > self.sector_count()? {
            return Err(DiskError::SectorDoesNotExist);
        }

        // Seek to the position of the sector, prep a sector-sized buffer
        self.file()
            .seek(SeekFrom::Start(self.sector_size()? as u64 * index as u64))?;
        let mut buffer: Vec<u8> = vec![0; self.sector_size()?];
        self.file().read_exact(&mut buffer)?;

        // Instantiate the correct sector type and return it
        let sector = match self.sector_size()? {
            128 => Sector::Small(Box::new(
                buffer
                    .as_slice()
                    .try_into()
                    .map_err(|_| DiskError::MismatchedDataLength)?,
            )),
            512 => Sector::Standard(Box::new(
                buffer
                    .as_slice()
                    .try_into()
                    .map_err(|_| DiskError::MismatchedDataLength)?,
            )),
            4096 => Sector::Large(Box::new(
                buffer
                    .as_slice()
                    .try_into()
                    .map_err(|_| DiskError::MismatchedDataLength)?,
            )),
            _ => return Err(DiskError::InvalidSectorSize),
        };
        Ok(sector)
    }
}

fn pad_to_nearest(data: &[u8]) -> Result<Vec<u8>, DiskError> {
    // Determine the nearest target size
    let target_size = if data.len() <= 128 {
        128
    } else if data.len() <= 512 {
        512
    } else if data.len() <= 4096 {
        4096
    } else {
        return Err(DiskError::MismatchedDataLength);
    };

    // Create a new Vec with the target size, initializing with zeros
    let mut padded_data = Vec::with_capacity(target_size);
    padded_data.extend_from_slice(data); // Copy the existing data

    // If necessary, pad with zeros
    padded_data.resize(target_size, 0);

    Ok(padded_data)
}
