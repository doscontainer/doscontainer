use crate::{error::DiskError, geometry::Geometry};

/// The CHS struct is almost identical to the Geometry struct. The purpose of having
/// two differently named structs this similar is to separate their roles. A Geometry
/// is used to indicate a fixed property of a virtualized disk. It's an immutable
/// given for any Disk struct since disks physical disks also don't change their characteristics.
///
/// CHS, in constrast, is an addressing mechanism that tells the operating system where
/// to find a specific sector on a disk. The CHS and Geometry structs may contain very
/// similar fields, they have completely different methods. The idea is that an operating
/// system (or `DOSContainer` itself) can look for sectors inside a Disk, and such a
/// sector is to be indicated by passing in a CHS parameter.
///
/// The CHS struct also contains conversion methods to go from itself to LBA addressing and
/// back. There is no separate LBA type (yet) because that's just a regular u32.
#[derive(Debug, PartialEq)]
pub struct CHS {
    cylinder: usize,
    head: usize,
    sector: usize,
}

impl CHS {
    pub fn new(cylinder: usize, head: usize, sector: usize) -> Self {
        CHS {
            cylinder,
            head,
            sector,
        }
    }

    /// Converts a Logical Block Address (LBA) into its corresponding
    /// Cylinder-Head-Sector (CHS) representation based on the given disk geometry.
    ///
    /// # Arguments
    ///
    /// * `lba` - A `u32` representing the logical block address to be converted.
    /// * `geometry` - A reference to a `Geometry` struct that provides information
    ///                about the disk geometry (cylinders, heads, sectors).
    ///
    /// # Returns
    ///
    /// Returns `Ok(CHS)` containing the cylinder, head, and sector corresponding to the given LBA
    /// if the conversion is successful. Returns `Err(DiskError)` if there is a problem with
    /// the disk geometry or if the LBA is out of range.
    ///
    /// # Errors
    ///
    /// * `DiskError::InvalidGeometry` - Returned if the disk geometry is invalid, i.e., if the
    ///   number of sectors per track or heads per cylinder is zero, or if the calculated
    ///   cylinder, head, or sector values are out of bounds.
    /// * `DiskError::SectorOutOfRange` - Returned if the provided LBA exceeds the total number
    ///   of sectors on the disk as determined by the geometry.
    ///
    pub fn from_lba(lba: u32, geometry: &Geometry) -> Result<Self, DiskError> {
        let sectors_per_track = u32::try_from(geometry.get_sectors())?;
        let heads_per_cylinder = u32::try_from(geometry.get_heads())?;
        let cylinders = u32::try_from(geometry.get_cylinders())?;

        // Check if sectors_per_track or heads_per_cylinder is zero
        if sectors_per_track == 0 || heads_per_cylinder == 0 {
            return Err(DiskError::InvalidGeometry);
        }

        // Calculate total sectors and check if the LBA is out of range
        let total_sectors = cylinders * sectors_per_track * heads_per_cylinder;
        if lba >= total_sectors {
            return Err(DiskError::SectorOutOfRange);
        }

        // Calculate cylinder, head, and sector values from LBA
        let cylinder = lba as usize / (heads_per_cylinder as usize * sectors_per_track as usize);
        let temp = lba % (heads_per_cylinder * sectors_per_track);
        let head = (temp / sectors_per_track) as usize;
        let sector = (temp % sectors_per_track + 1) as usize;

        // Check bounds against geometry
        if cylinder >= cylinders as usize
            || head >= heads_per_cylinder as usize
            || sector > sectors_per_track as usize
        {
            return Err(DiskError::InvalidGeometry);
        }

        // Return the CHS struct
        Ok(CHS::new(cylinder, head, sector))
    }

    /// Converts a Cylinder-Head-Sector (CHS) address into its corresponding
    /// Logical Block Address (LBA) representation based on the provided disk geometry.
    ///
    /// # Arguments
    ///
    /// * `geometry` - A reference to a `Geometry` struct that defines the disk geometry,
    ///                including the number of cylinders, heads, and sectors.
    ///
    /// # Returns
    ///
    /// Returns `Ok(u32)` containing the LBA address corresponding to the CHS values if the conversion
    /// is successful. Returns `Err(DiskError)` if the CHS values are out of range based on the disk geometry.
    ///
    /// # Errors
    ///
    /// * `DiskError::CylinderOutOfRange` - If the CHS cylinder value exceeds the number of cylinders
    ///   defined by the disk geometry.
    /// * `DiskError::HeadOutOfRange` - If the CHS head value exceeds the number of heads defined by
    ///   the disk geometry.
    /// * `DiskError::SectorOutOfRange` - If the CHS sector is 0 (sectors are 1-based) or if the CHS sector
    ///   value exceeds the number of sectors defined by the disk geometry.
    ///
    /// # Formula
    ///
    /// The conversion from CHS to LBA is done using the following formula:
    ///
    /// \[
    /// LBA = (C \times H_{\text{max}} \times S_{\text{max}}) + (H \times S_{\text{max}}) + (S - 1)
    /// \]
    ///
    /// Where:
    /// * `C` is the cylinder number (0-based)
    /// * `H` is the head number (0-based)
    /// * `S` is the sector number (1-based)
    /// * `H_{\text{max}}` is the maximum number of heads (as per the geometry)
    /// * `S_{\text{max}}` is the maximum number of sectors (as per the geometry)
    ///
    pub fn to_lba(&self, geometry: &Geometry) -> Result<u32, DiskError> {
        // Check for out-of-range values based on the geometry
        if geometry.get_cylinders() <= self.cylinder {
            return Err(DiskError::CylinderOutOfRange);
        }
        if geometry.get_heads() <= self.head {
            return Err(DiskError::HeadOutOfRange);
        }
        if self.sector == 0 || geometry.get_sectors() < self.sector {
            return Err(DiskError::SectorOutOfRange);
        }

        // Formula: LBA = (C * H_max * S_max) + (H * S_max) + (S - 1)
        let sectors_per_track = u32::try_from(geometry.get_sectors())?;
        let heads_per_cylinder = u32::try_from(geometry.get_heads())?;

        let lba = (u32::try_from(self.cylinder)? * heads_per_cylinder * sectors_per_track)
            + (u32::try_from(self.head)? * sectors_per_track)
            + (u32::try_from(self.sector)? - 1);

        Ok(lba)
    }
}

impl Default for CHS {
    fn default() -> Self {
        Self::new(0, 0, 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chs_lba_roundtrip() {
        let geometry = Geometry::new(40, 2, 12).unwrap();
        let address = CHS::new(2, 1, 4);
        let lba = address
            .to_lba(&geometry)
            .expect("Conversion to LBA failed.");
        let roundtripped = CHS::from_lba(lba, &geometry).unwrap();
        assert_eq!(address, roundtripped);
    }
}
