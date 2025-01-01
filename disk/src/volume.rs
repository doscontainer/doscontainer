use crate::error::DiskError;

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
}