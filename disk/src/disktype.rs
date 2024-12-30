use std::{fmt, str::FromStr};

use crate::{error::DiskError, geometry::Geometry};

/// Represents different types of disk formats, including various floppy disk types and a hard disk.
///
/// Each variant corresponds to a specific disk type, characterized by its storage capacity
/// and physical size.
#[derive(Clone, Debug, PartialEq)]
pub enum DiskType {
    /// 3.5-inch floppy disk with 720KB capacity.
    F35_720,
    /// 3.5-inch floppy disk with 1.44MB capacity.
    F35_1440,
    /// 3.5-inch floppy disk with 2.88MB capacity.
    F35_2880,
    /// 5.25-inch floppy disk with 160KB capacity.
    F525_160,
    /// 5.25-inch floppy disk with 180KB capacity.
    F525_180,
    /// 5.25-inch floppy disk with 320KB capacity.
    F525_320,
    /// 5.25-inch floppy disk with 360KB capacity.
    F525_360,
    /// 5.25-inch floppy disk with 1.2MB capacity.
    F525_1200,
    /// Represents a generic hard disk drive.
    HardDisk,
}

impl fmt::Display for DiskType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DiskType::F35_720 => write!(f, "3.5-inch floppy disk (720KB)"),
            DiskType::F35_1440 => write!(f, "3.5-inch floppy disk (1.44MB)"),
            DiskType::F35_2880 => write!(f, "3.5-inch floppy disk (2.88MB)"),
            DiskType::F525_160 => write!(f, "5.25-inch floppy disk (160KB)"),
            DiskType::F525_180 => write!(f, "5.25-inch floppy disk (180KB)"),
            DiskType::F525_320 => write!(f, "5.25-inch floppy disk (320KB)"),
            DiskType::F525_360 => write!(f, "5.25-inch floppy disk (360KB)"),
            DiskType::F525_1200 => write!(f, "5.25-inch floppy disk (1.2MB)"),
            DiskType::HardDisk => write!(f, "Generic hard disk drive"),
        }
    }
}

/// Represents different types of hard disks. A Disk always has a DiskType to indicate what it is.
/// Only if the DiskType is HardDisk you should also set a HardDiskType. The default HardDiskType
/// is CUSTOM so you can specify your own size or geometry in the manifest. The predefined types
/// are included to offer period appropriate configurations that IBM used to ship in their early
/// PC-compatible machine types.
#[derive(Debug)]
pub enum HardDiskType {
    IBMXT0,
    IBMXT1,
    IBMXT2,
    IBMXT3,
    IBMAT1,
    IBMAT2,
    IBMAT3,
    IBMAT4,
    IBMAT5,
    CUSTOM,
}


impl FromStr for HardDiskType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.to_ascii_uppercase().trim() {
            "IBMXT0" => HardDiskType::IBMXT0,
            "IBMXT1" => HardDiskType::IBMXT1,
            "IBMXT2" => HardDiskType::IBMXT2,
            "IBMXT3" => HardDiskType::IBMXT3,
            "IBMAT1" => HardDiskType::IBMAT1,
            "IBMAT2" => HardDiskType::IBMAT2,
            "IBMAT3" => HardDiskType::IBMAT3,
            "IBMAT4" => HardDiskType::IBMAT4,
            "IBMAT5" => HardDiskType::IBMAT5,
            _ => HardDiskType::CUSTOM, // Default to CUSTOM
        })
    }
}

impl HardDiskType {

    /// Get the physical geometries for a predefined hard disk type, or an invalid set for
    /// the CUSTOM type. If you use CUSTOM, you must set the geometry to something sensible yourself.
    pub fn geometry(&self) -> Result<Geometry, DiskError> {
        match self {
            Self::IBMXT0 => Ok(Geometry::new(306, 2, 17)?),
            Self::IBMXT1 => Ok(Geometry::new(376, 18, 17)?),
            Self::IBMXT2 => Ok(Geometry::new(306, 6, 17)?),
            Self::IBMXT3 => Ok(Geometry::new(306, 4, 17)?),
            Self::IBMAT1 => Ok(Geometry::new(306, 4, 17)?),
            Self::IBMAT2 => Ok(Geometry::new(615, 4, 17)?),
            Self::IBMAT3 => Ok(Geometry::new(615, 6, 17)?),
            Self::IBMAT4 => Ok(Geometry::new(940, 8, 17)?),
            Self::IBMAT5 => Ok(Geometry::new(940, 6, 17)?),
            Self::CUSTOM => Ok(Geometry::new(0, 0, 0)?),
        }
    }
}

impl DiskType {
    /// Returns the media descriptor byte associated with the disk type.
    ///
    /// The media descriptor is a single byte that identifies the type of media.
    ///
    /// # Returns
    ///
    /// * A `u8` value representing the media descriptor for the specific disk type.
    ///
    pub fn media_descriptor(&self) -> u8 {
        match self {
            Self::F35_720 => 0xF9,
            Self::F35_1440 => 0xF0,
            Self::F35_2880 => 0xF0,
            Self::F525_160 => 0xFE,
            Self::F525_180 => 0xFC,
            Self::F525_320 => 0xFF,
            Self::F525_360 => 0xFD,
            Self::F525_1200 => 0xF9,
            Self::HardDisk => 0xF8,
        }
    }

    /// Returns the FAT (File Allocation Table) media ID byte for a given disk type.
    ///
    /// This function provides the FAT ID byte associated with different disk formats.
    /// These values are typically used in FAT file systems to indicate the type of media
    /// being used (e.g., floppy disks, hard disks).
    ///
    /// # Return
    ///
    /// Returns an 8-bit unsigned integer (`u8`) representing the FAT ID byte.
    ///
    /// # FAT ID Values:
    /// - `0xF9` for `F35_720`, `F35_1440`, and `F35_1200` (3.5-inch floppy disks of different sizes).
    /// - `0xF0` for `F35_2880` (3.5-inch floppy disk with 2.88 MB capacity).
    /// - `0xFE`, `0xFD`, `0xFF`, and others for various 5.25-inch floppy disks (`F525_*` variants).
    /// - `0xF8` for hard disks (`HardDisk`).
    ///
    /// Some formats, like `F525_320`, may have multiple possible values (e.g., `0xFF` or `0xFA`),
    /// but `0xFF` is the default here.
    ///
    /// # Example
    ///
    pub fn fat_id(&self) -> u8 {
        match self {
            Self::F35_720 => 0xF9,
            Self::F35_1440 => 0xF9,
            Self::F35_2880 => 0xF0,
            Self::F525_160 => 0xFE,
            Self::F525_180 => 0xFC,
            Self::F525_320 => 0xFF, // 0xFA is also seen, unclear when.
            Self::F525_360 => 0xFD,
            Self::F525_1200 => 0xF9,
            Self::HardDisk => 0xF8,
        }
    }

    /// Returns the total number of sectors available on the disk.
    ///
    /// Each sector represents a fixed amount of data storage. This method returns the
    /// total count of sectors, which can be used to calculate the overall disk capacity.
    ///
    /// # Returns
    ///
    /// * `Some(usize)` containing the sector count if applicable, or `None` for a hard disk.
    ///
    pub fn sector_count(&self) -> Option<usize> {
        match self {
            Self::F35_720 => Some(1440),
            Self::F35_1440 => Some(2880),
            Self::F35_2880 => Some(5760),
            Self::F525_160 => Some(320),
            Self::F525_180 => Some(360),
            Self::F525_320 => Some(640),
            Self::F525_360 => Some(720),
            Self::F525_1200 => Some(2400),
            Self::HardDisk => None,
        }
    }

    /// Return a hard coded number for now, change when we handle large sectors!
    pub fn sector_size(&self) -> usize {
        512
    }
}

#[cfg(test)]
mod tests {
    use super::DiskType;

    #[test]
    fn get_media_descriptor() {
        assert_eq!(DiskType::F35_720.media_descriptor(), 0xF9);
        assert_eq!(DiskType::F35_1440.media_descriptor(), 0xF0);
        assert_eq!(DiskType::F35_2880.media_descriptor(), 0xF0);
        assert_eq!(DiskType::F525_160.media_descriptor(), 0xFE);
        assert_eq!(DiskType::F525_180.media_descriptor(), 0xFC);
        assert_eq!(DiskType::F525_320.media_descriptor(), 0xFF);
        assert_eq!(DiskType::F525_360.media_descriptor(), 0xFD);
        assert_eq!(DiskType::F525_1200.media_descriptor(), 0xF9);
        assert_eq!(DiskType::HardDisk.media_descriptor(), 0xF8);
    }

    #[test]
    fn get_sector_count() {
        assert_eq!(DiskType::F35_720.sector_count(), Some(1440));
        assert_eq!(DiskType::F35_1440.sector_count(), Some(2880));
        assert_eq!(DiskType::F35_2880.sector_count(), Some(5760));
        assert_eq!(DiskType::F525_160.sector_count(), Some(320));
        assert_eq!(DiskType::F525_180.sector_count(), Some(360));
        assert_eq!(DiskType::F525_320.sector_count(), Some(640));
        assert_eq!(DiskType::F525_360.sector_count(), Some(720));
        assert_eq!(DiskType::F525_1200.sector_count(), Some(2400));
        assert_eq!(DiskType::HardDisk.sector_count(), None);
    }

    #[test]
    fn test_f35_720() {
        let disk = DiskType::F35_720;
        assert_eq!(disk.fat_id(), 0xF9);
    }

    #[test]
    fn test_f35_1440() {
        let disk = DiskType::F35_1440;
        assert_eq!(disk.fat_id(), 0xF9);
    }

    #[test]
    fn test_f35_2880() {
        let disk = DiskType::F35_2880;
        assert_eq!(disk.fat_id(), 0xF0);
    }

    #[test]
    fn test_f525_160() {
        let disk = DiskType::F525_160;
        assert_eq!(disk.fat_id(), 0xFE);
    }

    #[test]
    fn test_f525_180() {
        let disk = DiskType::F525_180;
        assert_eq!(disk.fat_id(), 0xFC);
    }

    #[test]
    fn test_f525_320() {
        let disk = DiskType::F525_320;
        assert_eq!(disk.fat_id(), 0xFF);
    }

    #[test]
    fn test_f525_360() {
        let disk = DiskType::F525_360;
        assert_eq!(disk.fat_id(), 0xFD);
    }

    #[test]
    fn test_f525_1200() {
        let disk = DiskType::F525_1200;
        assert_eq!(disk.fat_id(), 0xF9);
    }

    #[test]
    fn test_hard_disk() {
        let disk = DiskType::HardDisk;
        assert_eq!(disk.fat_id(), 0xF8);
    }
}
