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

/// Just about everyone uses 512-byte sectors, implement the Default trait for that.
impl Default for SectorSize {
    fn default() -> Self {
        SectorSize::S512
    }
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

    pub fn get(&self) -> usize {
        self.as_usize()
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn default_is_512() {
        assert_eq!(SectorSize::default(), SectorSize::S512);
    }

    #[test]
    fn as_usize_returns_correct_values() {
        assert_eq!(SectorSize::S128.as_usize(), 128);
        assert_eq!(SectorSize::S256.as_usize(), 256);
        assert_eq!(SectorSize::S512.as_usize(), 512);
        assert_eq!(SectorSize::S1024.as_usize(), 1024);
        assert_eq!(SectorSize::S2048.as_usize(), 2048);
        assert_eq!(SectorSize::S4096.as_usize(), 4096);
    }

    #[test]
    fn try_from_valid_sizes() {
        assert_eq!(SectorSize::try_from(128).unwrap(), SectorSize::S128);
        assert_eq!(SectorSize::try_from(256).unwrap(), SectorSize::S256);
        assert_eq!(SectorSize::try_from(512).unwrap(), SectorSize::S512);
        assert_eq!(SectorSize::try_from(1024).unwrap(), SectorSize::S1024);
        assert_eq!(SectorSize::try_from(2048).unwrap(), SectorSize::S2048);
        assert_eq!(SectorSize::try_from(4096).unwrap(), SectorSize::S4096);
    }

    #[test]
    fn try_from_invalid_size_returns_error() {
        let err = SectorSize::try_from(100);
        assert!(err.is_err());
        assert_eq!(err.unwrap_err(), DiskError::InvalidSectorSize);
    }

    #[test]
    fn display_returns_expected_string() {
        assert_eq!(format!("{}", SectorSize::S128), "128 bytes");
        assert_eq!(format!("{}", SectorSize::S256), "256 bytes");
        assert_eq!(format!("{}", SectorSize::S512), "512 bytes");
        assert_eq!(format!("{}", SectorSize::S1024), "1024 bytes");
        assert_eq!(format!("{}", SectorSize::S2048), "2048 bytes");
        assert_eq!(format!("{}", SectorSize::S4096), "4096 bytes");
    }
}
