use crate::error::DiskError;

/// Represents a volume on a disk, defined by a range of sectors.
///
/// A `Volume` has a start and an end sector, with its size being the difference
/// between these two values. It provides methods to access the volume's boundaries
/// and read sectors within it.
///
/// # Methods
/// - `new`: Creates a new volume with a start and end sector. Returns an error
///   if the end sector is not greater than the start sector.
/// - `start_sector`: Returns the start sector of the volume.
/// - `end_sector`: Returns the end sector of the volume.
/// - `size`: Returns the size of the volume in sectors (i.e., the difference
///   between the end and start sector).
/// - `read_sector`: Reads a sector at a given index relative to the volume, returning
///   a `Sector` or a `DiskError` in case of failure.

#[derive(Debug)]
pub struct Volume {
    start_sector: usize,
    end_sector: usize,
}

impl Volume {
    /// Creates a new `Volume` instance with the specified start and end sectors.
    ///
    /// # Parameters
    /// - `start_sector`: The starting sector of the volume.
    /// - `end_sector`: The ending sector of the volume.
    ///
    /// # Returns
    /// - `Ok(Volume)` if the volume is valid.
    /// - `Err(DiskError::InvalidVolumeSize)` if the `end_sector` is not greater than
    ///   the `start_sector`.
    pub fn new(start_sector: usize, end_sector: usize) -> Result<Self, DiskError> {
        // Ensure the end sector is greater than the start sector
        if end_sector <= start_sector {
            return Err(DiskError::InvalidVolumeSize);
        }
        Ok(Volume {
            start_sector,
            end_sector,
        })
    }

    /// Returns the start sector of the volume.
    ///
    /// # Returns
    /// - `usize`: The start sector of the volume.
    pub fn start_sector(&self) -> usize {
        self.start_sector
    }

    /// Returns the end sector of the volume.
    ///
    /// # Returns
    /// - `usize`: The end sector of the volume.
    pub fn end_sector(&self) -> usize {
        self.end_sector
    }

    /// Returns the size of the volume in sectors.
    ///
    /// This is calculated as the difference between the `end_sector` and `start_sector`.
    ///
    /// # Returns
    /// - `usize`: The number of sectors in the volume.
    pub fn size(&self) -> usize {
        self.end_sector - self.start_sector
    }
}
