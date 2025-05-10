use crate::error::DiskError;

/// Represents a disk sector with different sizes: small, standard, and large.
///
/// A sector is a unit of data storage on a disk. This enum provides three variants:
///
/// - `Small`: A sector with a size of 128 bytes.
/// - `Standard`: A sector with a size of 512 bytes (most common).
/// - `Large`: A sector with a size of 4096 bytes (commonly used in larger media).
///
/// The `Sector` enum uses fixed-size arrays to represent sectors of various sizes.
#[derive(Debug, PartialEq)]
pub enum Sector {
    /// A sector with a size of 128 bytes.
    Small(Box<[u8; 128]>),

    /// A sector with a size of 512 bytes.
    Standard(Box<[u8; 512]>),

    /// A sector with a size of 4096 bytes.
    Large(Box<[u8; 4096]>),
}

impl Sector {
    /// Returns the size of the sector in bytes.
    pub fn size(&self) -> usize {
        match self {
            Sector::Small(_) => 128,
            Sector::Standard(_) => 512,
            Sector::Large(_) => 4096,
        }
    }

    /// Creates a new small sector with the given data.
    pub fn new_small(data: [u8; 128]) -> Self {
        Sector::Small(Box::new(data))
    }

    /// Creates a new standard sector with the given data.
    pub fn new_standard(data: [u8; 512]) -> Self {
        Sector::Standard(Box::new(data))
    }

    /// Creates a new large sector with the given data.
    pub fn new_large(data: [u8; 4096]) -> Self {
        Sector::Large(Box::new(data))
    }

    /// Extracts the raw byte data from the sector.
    pub fn data(&self) -> &[u8] {
        match self {
            Sector::Small(data) => &data[..],
            Sector::Standard(data) => &data[..],
            Sector::Large(data) => &data[..],
        }
    }

    pub fn from_slice(slice: &[u8]) -> Option<Self> {
        match slice.len() {
            128 => {
                let mut data = [0u8; 128];
                data.copy_from_slice(slice);
                Some(Sector::Small(Box::new(data)))
            }
            512 => {
                let mut data = [0u8; 512];
                data.copy_from_slice(slice);
                Some(Sector::Standard(Box::new(data)))
            }
            4096 => {
                let mut data = [0u8; 4096];
                data.copy_from_slice(slice);
                Some(Sector::Large(Box::new(data)))
            }
            _ => None,
        }
    }
}

impl AsRef<[u8]> for Sector {
    fn as_ref(&self) -> &[u8] {
        self.data()
    }
}

impl TryFrom<&[u8]> for Sector {
    type Error = DiskError;

    fn try_from(slice: &[u8]) -> Result<Self, Self::Error> {
        match slice.len() {
            128 => {
                let mut data = [0u8; 128];
                data.copy_from_slice(slice);
                Ok(Sector::Small(Box::new(data)))
            }
            512 => {
                let mut data = [0u8; 512];
                data.copy_from_slice(slice);
                Ok(Sector::Standard(Box::new(data)))
            }
            4096 => {
                let mut data = [0u8; 4096];
                data.copy_from_slice(slice);
                Ok(Sector::Large(Box::new(data)))
            }
            _other => Err(DiskError::InvalidSectorSize),
        }
    }
}
