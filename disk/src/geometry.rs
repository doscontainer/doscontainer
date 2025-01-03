use crate::DiskError;

/// Represents the geometry of a disk, specifically the number of cylinders, heads, and sectors.
///
/// This struct is used to describe the physical layout of a disk. It includes:
/// - `cylinders`: The number of cylinders on the disk (usually corresponding to tracks in older disk technologies).
/// - `heads`: The number of read/write heads on the disk (often corresponding to the number of platters).
/// - `sectors`: The number of sectors per track, which is a fixed number of data units on each track of a disk.
///
/// The `Geometry` struct supports several utility methods to convert this data into various formats,
/// including MBR partition table bytes and VHD disk format bytes. It also includes validation to ensure
/// the values for cylinders, heads, and sectors fall within reasonable and supported ranges for disk geometries.
#[derive(Clone, Debug)]
pub struct Geometry {
    cylinders: usize,
    heads: usize,
    sectors: usize,
}

impl Geometry {
    /// Creates a new `Geometry` instance with the specified number of cylinders, heads, and sectors.
    ///
    /// This function validates that the provided values for cylinders, heads, and sectors are within
    /// acceptable ranges:
    /// - `cylinders`: Must be between 1 and 1024 (inclusive).
    /// - `heads`: Must be between 1 and 255 (inclusive).
    /// - `sectors`: Must be between 8 and 63 (inclusive).
    ///
    /// If any of the values fall outside of these ranges, the function returns a `DiskError` with an
    /// appropriate error variant. If the values are valid, it creates and returns a `Geometry` instance.
    ///
    /// # Arguments
    /// - `cylinders`: The number of cylinders on the disk (tracks).
    /// - `heads`: The number of read/write heads on the disk.
    /// - `sectors`: The number of sectors per track on the disk.
    ///
    /// # Returns
    /// - `Ok(Geometry)`: If the provided values are valid, returns a new `Geometry` instance.
    /// - `Err(DiskError)`: If any of the values are out of range, returns an error indicating which
    ///   value is invalid (`CylinderOutOfRange`, `HeadOutOfRange`, or `SectorOutOfRange`).
    pub fn new(cylinders: usize, heads: usize, sectors: usize) -> Result<Geometry, DiskError> {
        if !(1..=1024).contains(&cylinders) {
            return Err(DiskError::CylinderOutOfRange);
        }
        if !(1..=255).contains(&heads) {
            return Err(DiskError::HeadOutOfRange);
        }
        if !(8..=63).contains(&sectors) {
            return Err(DiskError::SectorOutOfRange);
        }
        Ok(Geometry {
            cylinders,
            heads,
            sectors,
        })
    }

    /// Convert this struct into a three-byte array for use in an MBR partition table.
    ///
    /// This function converts the disk geometry (cylinders, heads, and sectors) into a compact
    /// three-byte format, as required by the MBR (Master Boot Record) partition table entry.
    ///
    /// The MBR format encodes the geometry as follows:
    /// - Byte 1: Head (8 bits)
    /// - Byte 2: A combination of the sector and overflow bits from the cylinders
    /// - Byte 3: The low byte of the cylinder value
    ///
    /// The cylinder value is split into two parts:
    /// - The low 8 bits of the cylinder are placed in the third byte.
    /// - The high 2 bits of the cylinder are placed in the second byte, combined with the sector information.
    ///
    /// # Returns
    /// - `Ok([u8; 3])`: A 3-byte array representing the disk geometry in MBR format.
    /// - `Err(DiskError)`: If the conversion fails (e.g., invalid sector size or other errors), an error is returned.
    ///
    /// # Logic Details
    /// - `sector_bits`: Extracts the lower 6 bits of the sector to fit it within the 6-bit limit in the MBR format.
    /// - `cylinder_byte`: The low byte of the cylinder (lower 8 bits).
    /// - `sector_byte`: Combines the lower 6 bits of the sector with the high 2 bits of the cylinder to form the second byte.
    pub fn to_mbr_bytes(&self) -> Result<[u8; 3], DiskError> {
        // Convert sectors, heads, and cylinders into the appropriate byte sizes.
        let sector: u8 = self.sectors.try_into()?; // Convert sector to u8
        let head: u8 = self.heads.try_into()?; // Convert head count to u8
        let cylinder: u16 = self.cylinders.try_into()?; // Convert cylinder count to u16

        // Extract the lower 6 bits of the sector (only the lower 6 are used in MBR format).
        let sector_bits = sector & 0b0011_1111; // Mask out the upper 2 bits of sector

        // Extract the lower 8 bits of the cylinder, which will be the third byte.
        let cylinder_byte = (cylinder & 0b0000_0000_1111_1111) as u8; // Lower 8 bits of the cylinder

        // Extract the upper 2 bits of the cylinder for inclusion in the second byte.
        let cylinder_overflow_bits = cylinder & 0b0000_0011_0000_0000; // Upper 2 bits of the cylinder

        // Combine the sector bits and the upper 2 bits of the cylinder to form the second byte.
        let sector_byte = sector_bits | (cylinder_overflow_bits >> 2) as u8; // Shift and combine for second byte

        // Return the three-byte array containing the head, sector byte, and cylinder byte.
        Ok([head, sector_byte, cylinder_byte])
    }

    /// Convert 3 bytes from an MBR partition table into a Geometry instance.
    /// This is the inverse operation of `to_mbr_bytes()`.
    ///
    /// The MBR format uses 3 bytes to encode the geometry of a disk, including the cylinder,
    /// head, and sector values. This function takes the 3-byte array and decodes it back
    /// into the `Geometry` struct, which represents the disk geometry in terms of cylinders,
    /// heads, and sectors.
    ///
    /// The MBR format stores the geometry as follows:
    /// - Byte 1: Head (8 bits)
    /// - Byte 2: A combination of the sector (lower 6 bits) and the overflow bits from the cylinder (upper 2 bits)
    /// - Byte 3: The low byte of the cylinder value (8 bits)
    ///
    /// # Returns
    /// - `Ok(Geometry)`: A `Geometry` instance with the decoded values for cylinders, heads, and sectors.
    /// - `Err(DiskError)`: If the values cannot be converted into a valid `Geometry` (e.g., invalid range for geometry values).
    ///
    /// # Logic Details:
    /// - `sector_byte & 0b1100_0000`: Extracts the upper 2 bits of the second byte, which are part of the cylinder value.
    /// - `(sector_byte & 0b0011_1111)`: Extracts the lower 6 bits of the second byte, which represent the sector value.
    /// - The `cylinder` is reconstructed by combining the upper 2 bits from the second byte with the 8 bits from the third byte.
    pub fn from_mbr_bytes(bytes: [u8; 3]) -> Result<Self, DiskError> {
        // Extract the head value from the first byte (head is represented directly).
        let head = bytes[0];

        // Extract the second byte, which contains both the sector and the upper 2 bits of the cylinder.
        let sector_byte = bytes[1];

        // Reconstruct the cylinder value by combining the upper 2 bits from the sector byte and the full 8 bits from the third byte.
        // - `sector_byte & 0b1100_0000`: Extracts the upper 2 bits of the sector byte, which are part of the cylinder value.
        // - `(bytes[2] as u16)`: The full cylinder byte, representing the lower 8 bits of the cylinder.
        let cylinder = (((sector_byte & 0b1100_0000) as u16) << 2) | (bytes[2] as u16);

        // Extract the sector value from the lower 6 bits of the second byte.
        // - `sector_byte & 0b0011_1111`: Extracts the lower 6 bits, which correspond to the sector value.
        let sector = sector_byte & 0b0011_1111;

        // Create a new Geometry instance from the decoded values for cylinder, head, and sector.
        let geometry = Geometry::new(
            usize::from(cylinder), // Convert cylinder to usize
            usize::from(head),     // Convert head to usize
            usize::from(sector),   // Convert sector to usize
        )?;

        // Return the decoded Geometry instance.
        Ok(geometry)
    }

    /// Convert the geometry into the 4 bytes required for the VHD disk format footer field.
    /// This conversion is different from the MBR partition entry calculation.
    ///
    /// The VHD (Virtual Hard Disk) format requires a 4-byte footer field to represent the
    /// geometry of a virtual disk. The format is as follows:
    /// - The first two bytes represent the cylinder value (most significant 2 bytes).
    /// - The third byte represents the number of heads (converted to a byte).
    /// - The fourth byte represents the number of sectors (converted to a byte).
    ///
    /// # Returns
    /// - `Ok([u8; 4])`: A 4-byte array representing the geometry of the disk in the VHD format.
    /// - `Err(DiskError)`: If any value cannot be converted into a valid byte (e.g., out of range for heads or sectors).
    ///
    /// # Logic Details:
    /// - The cylinder value is first converted to a 2-byte array using `to_be_bytes()`.
    /// - The first 2 bytes of the cylinder are added to the 4-byte array.
    /// - The `heads` and `sectors` values are converted to bytes using `try_into()` and added to the 4-byte array.
    pub fn to_vhd_bytes(&self) -> Result<[u8; 4], DiskError> {
        // Convert the cylinder value into two bytes, in big-endian order.
        let cylinder_bytes = self.cylinders.to_be_bytes();

        // Construct the 4-byte array:
        // - First 2 bytes are from the cylinder value (most significant bytes).
        // - Third byte is the number of heads, converted to a byte.
        // - Fourth byte is the number of sectors, converted to a byte.
        Ok([
            cylinder_bytes[0],        // Most significant byte of the cylinder
            cylinder_bytes[1],        // Least significant byte of the cylinder
            self.heads.try_into()?,   // Convert heads to byte (u8)
            self.sectors.try_into()?, // Convert sectors to byte (u8)
        ])
    }

    /// Convert the 4 bytes from the VHD disk format into CHS (Cylinder-Head-Sector) values.
    /// This is the inverse operation of `to_vhd_bytes()`, used to reconstruct the disk geometry
    /// from the byte format in the VHD (Virtual Hard Disk) footer field.
    ///
    /// # Parameters
    /// - `bytes`: A 4-byte array representing the VHD footer field values for cylinders, heads, and sectors.
    ///
    /// # Returns
    /// - `Geometry`: A `Geometry` struct that holds the reconstructed values for cylinders, heads, and sectors.
    ///
    /// # Notes
    /// - The first two bytes (bytes[0] and bytes[1]) represent the cylinders, stored as a big-endian 16-bit value.
    /// - The third byte (bytes[2]) represents the heads.
    /// - The fourth byte (bytes[3]) represents the sectors.
    pub fn from_vhd_bytes(bytes: [u8; 4]) -> Geometry {
        // The first two bytes represent the cylinder value, which is a 16-bit unsigned integer
        // in big-endian byte order.
        let cylinders = usize::from(u16::from_be_bytes([bytes[0], bytes[1]]));

        // The third byte represents the number of heads.
        let heads = usize::from(bytes[2]);

        // The fourth byte represents the number of sectors.
        let sectors = usize::from(bytes[3]);

        // Create and return a Geometry instance using the reconstructed values.
        Geometry {
            cylinders,
            heads,
            sectors,
        }
    }

    /// Get the number of cylinders (tracks) on the disk.
    ///
    /// This method returns the number of cylinders, which is often used in disk geometries
    /// to represent the total number of tracks on all the platters of a disk. In older disk
    /// technologies, the cylinder represents a set of tracks that are aligned across all platters.
    ///
    /// # Returns
    /// - `usize`: The number of cylinders (tracks) on the disk.
    pub fn cylinders(&self) -> usize {
        self.cylinders
    }

    /// Get the number of heads (read/write heads) on the disk.
    ///
    /// This method returns the number of heads on the disk, which typically corresponds to
    /// the number of platters in a hard disk drive or the number of heads available for reading
    /// and writing data from different sides of a platter.
    ///
    /// # Returns
    /// - `usize`: The number of heads (read/write heads) on the disk.
    pub fn heads(&self) -> usize {
        self.heads
    }

    /// Get the number of sectors per track on the disk.
    ///
    /// This method returns the number of sectors on a single track of the disk. A sector is
    /// the smallest unit of storage on a disk, and the number of sectors on each track is
    /// typically constant for the disk geometry.
    ///
    /// # Returns
    /// - `usize`: The number of sectors per track on the disk.
    pub fn sectors(&self) -> usize {
        self.sectors
    }
}

#[cfg(test)]
mod tests {
    use super::Geometry;

    /// Test that the conversion of a `Geometry` instance to MBR bytes and back to a `Geometry`
    /// instance works correctly, ensuring data integrity during the roundtrip.
    ///
    /// This test checks the following:
    /// - A `Geometry` instance is successfully created with valid cylinder, head, and sector values.
    /// - The `to_mbr_bytes()` function correctly converts the `Geometry` into a byte array suitable for an MBR partition entry.
    /// - The byte array is converted back into a `Geometry` instance using `from_mbr_bytes()`.
    /// - The converted `Geometry` instance, when passed through `to_mbr_bytes()` again, matches the original byte array.
    ///
    /// # Notes
    /// - The test uses known valid geometry values: 540 cylinders, 15 heads, and 63 sectors.
    /// - It ensures that the MBR byte conversion is symmetric (i.e., no data loss or corruption occurs during the conversion).
    #[test]
    fn mbr_bytes_roundtrip() {
        // Create a Geometry instance with 540 cylinders, 15 heads, and 63 sectors
        let geometry = Geometry::new(540, 15, 63);
        assert!(geometry.is_ok()); // Assert that the geometry creation was successful

        // Convert the Geometry instance to MBR bytes and verify it matches the expected byte array
        assert_eq!(geometry.unwrap().to_mbr_bytes().unwrap(), [15, 191, 28]);

        // Convert the MBR byte array back into a Geometry instance and verify it is valid
        let newgeom = Geometry::from_mbr_bytes([15, 191, 28]);
        assert!(newgeom.is_ok()); // Assert that the byte-to-geometry conversion was successful

        // Verify that the converted Geometry instance matches the original byte array when converted back to MBR bytes
        assert_eq!(newgeom.unwrap().to_mbr_bytes().unwrap(), [15, 191, 28]);
    }
}
