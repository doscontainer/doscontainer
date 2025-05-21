use std::fmt;

use crate::error::DiskError;

#[derive(Clone, Copy, Debug, PartialEq)]
/// Sector sizes are limited to values that appeared in relevant documentation
/// at some point in the PC platform's history. It's overly broad, but we're
/// building an abstraction here. This enum serves to limit the choices to settings
/// that make sense.
pub enum SectorSize {
    S128,
    S256,
    S512,
    S1024,
    S2048,
    S4096,
}

impl SectorSize {
    pub fn as_usize(&self) -> usize {
        match self {
            SectorSize::S128 => 128,
            SectorSize::S256 => 256,
            SectorSize::S512 => 512,
            SectorSize::S1024 => 1024,
            SectorSize::S2048 => 2048,
            SectorSize::S4096 => 4096,
        }
    }
}

impl fmt::Display for SectorSize {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} bytes", self.as_usize())
    }
}

impl TryFrom<usize> for SectorSize {
    type Error = DiskError;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            128 => Ok(SectorSize::S128),
            256 => Ok(SectorSize::S256),
            512 => Ok(SectorSize::S512),
            1024 => Ok(SectorSize::S1024),
            2048 => Ok(SectorSize::S2048),
            4096 => Ok(SectorSize::S4096),
            _ => Err(DiskError::InvalidSectorSize),
        }
    }
}