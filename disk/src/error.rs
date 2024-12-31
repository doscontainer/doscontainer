use std::fmt;
use std::num::TryFromIntError;

/// Represents the possible errors in the disk.
#[derive(Debug)]
pub enum DiskError {
    ConversionError,
    CylinderOutOfRange,
    HeadOutOfRange,
    InvalidDiskType,
    InvalidGeometry,
    InvalidSectorSize,
    IoError(std::io::Error),
    MismatchedDataLength,
    SectorDoesNotExist,
    SectorOutOfRange,
    UnsupportedDiskType,
    UnsupportedOperatingSystem,
}

impl fmt::Display for DiskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskError::ConversionError => write!(f, "Conversion error."),
            DiskError::CylinderOutOfRange => write!(f, "Cylinder out of range."),
            DiskError::HeadOutOfRange => write!(f, "Head of out range."),
            DiskError::InvalidDiskType => write!(f, "You cannot use this type of disk here."),
            DiskError::InvalidGeometry => write!(f, "Invalid geometry."),
            DiskError::InvalidSectorSize => write!(f, "Invalid sector size."),
            DiskError::MismatchedDataLength => {
                write!(f, "Trying to write an invalid amount of data.")
            }
            DiskError::SectorDoesNotExist => {
                write!(f, "Tried to access a sector that does not exist.")
            }
            DiskError::SectorOutOfRange => write!(f, "Sector out of range."),
            DiskError::UnsupportedDiskType => write!(f, "Unsupported disk type."),
            DiskError::UnsupportedOperatingSystem => write!(f, "Unsupported operating system."),
            DiskError::IoError(_) => write!(f, "IO Error!"),
        }
    }
}

impl From<std::io::Error> for DiskError {
    fn from(err: std::io::Error) -> Self {
        DiskError::IoError(err)
    }
}

impl From<TryFromIntError> for DiskError {
    fn from(_value: TryFromIntError) -> Self {
        DiskError::ConversionError
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disk_error_wrongtype_debug_format() {
        let error = DiskError::InvalidDiskType;
        let debug_str = format!("{:?}", error);
        assert_eq!(debug_str, "InvalidDiskType");
    }

    #[test]
    fn disk_error_mismatcheddata_debug_format() {
        let error = DiskError::MismatchedDataLength;
        let debug_str = format!("{:?}", error);
        assert_eq!(debug_str, "MismatchedDataLength")
    }

    #[test]
    fn disk_error_display_format() {
        let error = DiskError::InvalidDiskType;
        let display_str = format!("{}", error);
        assert_eq!(display_str, "You cannot use this type of disk here.");
    }
}