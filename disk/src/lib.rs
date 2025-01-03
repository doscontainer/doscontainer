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

    /// Adds a volume to the disk based on its type.
    ///
    /// This function handles the addition of a new volume to the disk. The behavior of the
    /// volume creation varies depending on the type of disk:
    ///
    /// - **Floppy disks**: For known floppy disk types (e.g., `F35_1440`, `F35_2880`, etc.),
    ///   a single volume spanning the entire disk is created. The `start_sector` and `end_sector`
    ///   arguments are ignored for these types, as they always use the full capacity of the disk.
    ///   An error will be returned if a volume already exists on the disk.
    ///
    /// - **Hard disks**: Hard disks are not yet supported, but the functionality for adding
    ///   multiple volumes will be implemented in the future. The current implementation for
    ///   `HardDisk` types simply marks the code with a `todo!()` placeholder.
    ///
    /// # Arguments
    ///
    /// - `_start_sector`: The starting sector of the volume to be added (currently unused for floppies).
    /// - `_end_sector`: The ending sector of the volume to be added (currently unused for floppies).
    ///
    /// # Returns
    ///
    /// Returns a `Result`:
    /// - `Ok(())` if the volume was successfully added.
    /// - `Err(DiskError)` if there was an error (e.g., a volume already exists or an invalid size).
    ///
    /// # Errors
    ///
    /// - `DiskError::VolumeAlreadyExists` if a volume already exists on the disk (for floppy disks).
    /// - `DiskError::InvalidVolumeSize` if the disk size is invalid for volume creation (for floppy disks).
    /// - `DiskError::NotImplemented` for `HardDisk` support, as the functionality is not yet implemented.
    fn add_volume(&mut self, _start_sector: usize, _end_sector: usize) -> Result<(), DiskError> {
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
            self.write_lba(
                sector.try_into().map_err(|_| DiskError::SectorOutOfRange)?,
                &data,
            )?;
        }
        Ok(())
    }

    /// Writes a sector to the specified volume on the disk.
    ///
    /// # Parameters
    /// - `volume_index`: The index of the volume to write to. This corresponds to the volume's position
    ///   in the disk's list of volumes.
    /// - `sector_index`: The sector index within the volume to write to.
    /// - `data`: A byte slice containing the data to write. The size of the data must match the disk's
    ///   sector size.
    ///
    /// # Returns
    /// - `Ok(())`: The write operation was successful.
    /// - `Err(DiskError)`: An error occurred during the operation. Possible errors include:
    ///     - `DiskError::VolumeDoesNotExist`: The specified `volume_index` does not correspond to a
    ///       valid volume.
    ///     - `DiskError::SectorOutOfRange`: The specified `sector_index` is out of bounds for the
    ///       selected volume or the computed logical block address (LBA) exceeds disk limits.
    ///     - `DiskError::SectorOverflow`: The size of the provided `data` exceeds the disk's sector size.
    ///
    /// # Errors
    /// This function performs several validation checks and may return the following errors:
    /// - **Volume existence**: If `volume_index` does not reference a valid volume, a
    ///   `DiskError::VolumeDoesNotExist` error is returned.
    /// - **Sector bounds**: If `sector_index` is greater than or equal to the number of sectors in the
    ///   selected volume, a `DiskError::SectorOutOfRange` error is returned.
    /// - **Sector size**: If the size of the `data` slice exceeds the disk's sector size, a
    ///   `DiskError::SectorOverflow` error is returned.
    fn volume_write_sector(
        &mut self,
        volume_index: usize,
        sector_index: usize,
        data: &[u8],
    ) -> Result<(), DiskError> {
        // Ensure we have a volume at the requested index
        if self.volumes().len() <= volume_index {
            return Err(DiskError::VolumeDoesNotExist);
        }

        let volume = &self.volumes()[volume_index];

        // Ensure we're writing within bounds of the volume itself
        if sector_index > volume.size() {
            return Err(DiskError::SectorOutOfRange);
        }

        // Ensure the data fits the disk's sector size
        if data.len() > self.sector_size()? {
            return Err(DiskError::SectorOverflow);
        }

        // Perform the sector translation
        let lba = volume
            .start_sector()
            .checked_add(sector_index)
            .ok_or(DiskError::SectorOutOfRange)?;

        // Perform the write
        self.write_lba(lba.try_into()?, data)?;
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

    /// Writes a sector to the specified LBA address (index) on the disk.
    ///
    /// This function pads the provided data to the nearest valid sector size,
    /// then writes it to the specified Logical Block Address (LBA) on the disk.
    /// It calculates the position using the `sector_size` and `index`, and seeks
    /// to the appropriate location before writing the data.
    ///
    /// # Parameters
    /// - `index`: The logical block address (LBA) index of the sector to write to.
    /// - `data`: A byte slice containing the data to write. The size will be padded
    ///   to the nearest valid sector size (128, 512, or 4096 bytes) before writing.
    ///
    /// # Returns
    /// - `Ok(())`: If the write operation completes successfully.
    /// - `Err(DiskError)`: If an error occurs during the write operation, such as:
    ///     - `DiskError::SectorOverflow`: If the padded data exceeds the disk's sector size.
    ///     - `DiskError::SeekError`: If the seek operation fails.
    ///     - `DiskError::WriteError`: If the write operation fails.
    fn write_lba(&mut self, index: u32, data: &[u8]) -> Result<(), DiskError> {
        // Get sector size once to avoid redundant calls
        let sector_size = self.sector_size()?;

        // Pad the data to match the sector size
        let padded_data = Self::pad_to_nearest(data)?;

        // Ensure the padded data fits within the sector size
        if padded_data.len() > sector_size {
            return Err(DiskError::SectorOverflow);
        }

        // Calculate the correct position in the file and seek to it
        self.file()
            .seek(SeekFrom::Start(sector_size as u64 * index as u64))
            .map_err(|_| DiskError::SeekError)?;

        // Write the padded data to the disk
        self.file()
            .write_all(&padded_data)
            .map_err(|_| DiskError::WriteError)?;

        Ok(())
    }

    /// Reads a sector from the specified LBA address (index) on the disk.
    ///
    /// This function checks if the given LBA index is valid, seeks to the correct position
    /// on the disk, and reads the sector data into a buffer. It then returns the sector
    /// as a specific type depending on the sector size.
    ///
    /// # Parameters
    /// - `index`: The logical block address (LBA) index of the sector to read.
    ///
    /// # Returns
    /// - `Ok(Sector)`: If the read operation is successful, it returns the sector.
    /// - `Err(DiskError)`: If an error occurs during the read operation, such as:
    ///     - `DiskError::SectorDoesNotExist`: If the given index is out of bounds.
    ///     - `DiskError::ReadError`: If an error occurs during reading from the disk.
    ///     - `DiskError::InvalidSectorSize`: If the sector size is not valid.
    fn read_lba(&self, index: u32) -> Result<Sector, DiskError> {
        // Get sector size and validate bounds
        let sector_size = self.sector_size()?;
        let sector_count = self.sector_count()?;

        if index as usize >= sector_count {
            return Err(DiskError::SectorDoesNotExist);
        }

        // Seek to the sector's location on disk
        self.file()
            .seek(SeekFrom::Start(sector_size as u64 * index as u64))
            .map_err(|_| DiskError::SeekError)?;

        // Prepare a buffer to hold the sector data
        let mut buffer = Vec::with_capacity(sector_size);
        self.file()
            .read_exact(&mut buffer)
            .map_err(|_| DiskError::ReadError)?;

        // Check if the buffer size matches the expected sector size
        if buffer.len() != sector_size {
            return Err(DiskError::MismatchedDataLength);
        }

        // Return the sector based on the sector size
        let sector = match sector_size {
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

    /// Pads the given data to the nearest valid sector size.
    ///
    /// This function ensures that the data is padded to fit one of the following
    /// sector sizes: 128, 512, or 4096 bytes. If the data exceeds 4096 bytes in
    /// length, an error is returned.
    ///
    /// The padding is done by resizing the provided data slice to the nearest
    /// sector size, and the new space is filled with zero bytes.
    ///
    /// # Parameters
    ///
    /// - `data`: A byte slice that contains the data to be padded. The function
    ///   will return a padded `Vec<u8>` to match one of the valid sector sizes.
    ///
    /// # Returns
    ///
    /// - `Ok(Vec<u8>)`: A `Vec<u8>` containing the original data padded to the
    ///   nearest valid sector size.
    /// - `Err(DiskError::MismatchedDataLength)`: If the data is larger than 4096
    ///   bytes, it cannot be padded to a valid sector size, and an error is returned.
    fn pad_to_nearest(data: &[u8]) -> Result<Vec<u8>, DiskError> {
        // Determine the nearest target size
        let target_size = match data.len() {
            len if len <= 128 => 128,
            len if len <= 512 => 512,
            len if len <= 4096 => 4096,
            _ => return Err(DiskError::MismatchedDataLength),
        };

        // If the data is already the target size, return it as-is.
        if data.len() == target_size {
            return Ok(data.to_vec());
        }

        // Create a new Vec with the target size and initialize with zeros, if necessary
        let mut padded_data = Vec::with_capacity(target_size);
        padded_data.extend_from_slice(data);

        // If necessary, pad with zeros
        padded_data.resize_with(target_size, Default::default);

        Ok(padded_data)
    }
}
