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
    Small([u8; 128]),

    /// A sector with a size of 512 bytes.
    Standard([u8; 512]),

    /// A sector with a size of 4096 bytes.
    Large([u8; 4096]),
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
        Sector::Small(data)
    }

    /// Creates a new standard sector with the given data.
    pub fn new_standard(data: [u8; 512]) -> Self {
        Sector::Standard(data)
    }

    /// Creates a new large sector with the given data.
    pub fn new_large(data: [u8; 4096]) -> Self {
        Sector::Large(data)
    }

    /// Extracts the raw byte data from the sector.
    pub fn data(&self) -> &[u8] {
        match self {
            Sector::Small(data) => &data[..],
            Sector::Standard(data) => &data[..],
            Sector::Large(data) => &data[..],
        }
    }
}
