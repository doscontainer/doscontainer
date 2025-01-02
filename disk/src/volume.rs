use crate::{error::DiskError, sector::Sector, Disk};

#[derive(Debug)]
pub struct Volume {
    start_sector: usize,
    end_sector: usize,
}

impl Volume {
    pub fn new(start_sector: usize, end_sector: usize) -> Result<Self, DiskError> {
        if end_sector <= start_sector {
            return Err(DiskError::InvalidVolumeSize);
        }
        Ok(Volume {
            start_sector,
            end_sector,
        })
    }

    /// Returns the start sector of the volume.
    pub fn start_sector(&self) -> usize {
        self.start_sector
    }

    /// Returns the end sector of the volume.
    pub fn end_sector(&self) -> usize {
        self.end_sector
    }

    /// Returns the size of the volume in sectors.
    pub fn size(&self) -> usize {
        self.end_sector - self.start_sector
    }

    /// Reads a sector at a given index relative to the volume itself, not the disk.
    ///
    /// This method calculates the absolute sector address (LBA) on the underlying disk
    /// based on the relative index within the volume and delegates the actual read
    /// operation to the `Disk::read_lba` method. It ensures that the read operation
    /// respects the volume's boundaries.
    ///
    /// # Parameters
    /// - `disk`: A reference to a `Disk` implementation from which the data will be read.
    /// - `index`: The zero-based index of the sector within the volume to read.
    ///
    /// # Returns
    /// - `Ok(Sector)`: If the sector is successfully read.
    /// - `Err(DiskError)`: If any of the following errors occur:
    ///   - [`DiskError::SectorOutOfRange`]: The `index` is outside the volume's bounds or an overflow occurs when calculating the absolute LBA.
    ///   - Errors propagated from the `Disk::read_lba` method.
    ///
    /// # Errors
    /// - This method ensures that:
    ///   - The `index` is within the valid range of the volume.
    /// - If the `index` is out of range or any other issue occurs during the read operation, an appropriate `DiskError` is returned.
    ///
    /// # Calculation
    /// The absolute LBA (Logical Block Address) is calculated as:
    /// ```text
    /// LBA = start_sector + index
    /// ```
    /// where `start_sector` is the starting sector of the volume on the disk.
    pub fn read_sector<D: Disk>(&self, disk: &D, index: usize) -> Result<Sector, DiskError> {
        // Ensure the index is within the bounds of the volume
        if index >= self.size() {
            return Err(DiskError::SectorOutOfRange);
        }

        // Calculate the absolute LBA address on the Disk
        let lba = self
            .start_sector
            .checked_add(index)
            .ok_or(DiskError::SectorOutOfRange)?;

        Ok(disk.read_lba(lba.try_into()?)?)
    }
}
