use crate::DiskError;

#[derive(Clone, Debug)]
pub struct Geometry {
    cylinders: usize,
    heads: usize,
    sectors: usize,
}

impl Geometry {
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
    pub fn to_mbr_bytes(&self) -> Result<[u8; 3], DiskError> {
        let sector: u8 = self.sectors.try_into()?;
        let head: u8 = self.heads.try_into()?;
        let cylinder: u16 = self.cylinders.try_into()?;

        let sector_bits = sector & 0b0011_1111;
        let cylinder_byte = (cylinder & 0b0000_0000_1111_1111) as u8;
        let cylinder_overflow_bits = cylinder & 0b0000_0011_0000_0000;
        let sector_byte = sector_bits | (cylinder_overflow_bits >> 2) as u8;
        Ok([head, sector_byte, cylinder_byte])
    }

    /// Convert 3 bytes from an MBR partition table into a Geometry instance.
    /// This is the inverse operation of `to_mbr_bytes()`.
    pub fn from_mbr_bytes(bytes: [u8; 3]) -> Result<Self, DiskError> {
        let head = bytes[0];
        let sector_byte = bytes[1];
        let cylinder = (((sector_byte & 0b1100_0000) as u16) << 2) | (bytes[2] as u16);
        let sector = sector_byte & 0b0011_1111;
        let geometry = Geometry::new(cylinder.into(), head.into(), sector.into())?;
        Ok(geometry)
    }

    /// Convert the geometry into the 4 bytes requred for the VHD disk format footer field.
    /// This conversion is different from the MBR partition entry calculation.
    pub fn to_vhd_bytes(&self) -> Result<[u8; 4], DiskError> {
        let cylinder_bytes = self.cylinders.to_be_bytes();
        Ok([
            cylinder_bytes[0],
            cylinder_bytes[1],
            self.heads.try_into()?,
            self.sectors.try_into()?,
        ])
    }

    /// Convert the 4 bytes from the VHD disk format into CHS values.
    /// This is the inverse operation of `to_vhd_bytes()'.
    pub fn from_vhd_bytes(bytes: [u8; 4]) -> Geometry {
        let cylinders = usize::from(u16::from_be_bytes([bytes[0], bytes[1]]));
        let heads = usize::from(bytes[2]);
        let sectors = usize::from(bytes[3]);

        Geometry {
            cylinders,
            heads,
            sectors,
        }
    }

    pub fn get_cylinders(&self) -> usize {
        self.cylinders
    }

    pub fn get_heads(&self) -> usize {
        self.heads
    }

    pub fn get_sectors(&self) -> usize {
        self.sectors
    }
}

#[cfg(test)]
mod tests {
    use super::Geometry;

    #[test]
    fn mbr_bytes_roundtrip() {
        let geometry = Geometry::new(540, 15, 63);
        assert!(geometry.is_ok());
        assert_eq!(geometry.unwrap().to_mbr_bytes().unwrap(), [15, 191, 28]);
        let newgeom = Geometry::from_mbr_bytes([15, 191, 28]);
        assert!(newgeom.is_ok());
        assert_eq!(newgeom.unwrap().to_mbr_bytes().unwrap(), [15, 191, 28]);
    }
}
