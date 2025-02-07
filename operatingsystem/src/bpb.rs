use disk::disktype::DiskType;

use crate::{error::OsError, OperatingSystem};

/// BIOS Parameter Block structure. Intuitively this should live with
/// Disk, but there's a lot more dependency on the operating system that
/// determines how a BPB gets translated onto on-disk bytes so we keep it
/// here for now.
pub struct BPB {
    bytes_per_sector: u16,
    sectors_per_cluster: u8,
    reserved_sectors: u16,
    fat_copies: u8,
    rootdir_entries: u16,
    sector_count: u16,
    media_descriptor: u8,
    sectors_per_fat: u16,
}

impl BPB {
    /// Public interface to convert the BPB into on-disk bytes. This method calls into
    /// private methods that do the actual work based on the operating system in use.
    ///
    /// The method returns the corresponding byte sequence for the specified operating system's BPB,
    /// or an error if the BPB is not applicable or the operating system is unsupported.
    ///
    /// # Errors:
    /// - `OsError::BpbNotApplicable` if the BPB is not relevant for the operating system (e.g., IBM PC-DOS 1.00 or 1.10).
    /// - `OsError::UnsupportedOs` if the operating system is not supported.
    pub fn as_bytes(&self, operating_system: &OperatingSystem) -> Result<Vec<u8>, OsError> {
        match operating_system {
            OperatingSystem::IBMDOS200 => Ok(self.as_pcdos_200_bytes()),
            OperatingSystem::IBMDOS100 | OperatingSystem::IBMDOS110 => {
                Err(OsError::BpbNotApplicable)
            }
            _ => Err(OsError::UnsupportedOs),
        }
    }

    /// Convert the BPB struct into on-disk bytes that correspond to IBM PC-DOS 2.00
    fn as_pcdos_200_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(13);
        bytes.extend_from_slice(&self.bytes_per_sector.to_le_bytes());
        bytes.push(self.sectors_per_cluster);
        bytes.extend_from_slice(&self.reserved_sectors.to_le_bytes());
        bytes.push(self.fat_copies);
        bytes.extend_from_slice(&self.rootdir_entries.to_le_bytes());
        bytes.extend_from_slice(&self.sector_count.to_le_bytes());
        bytes.push(self.media_descriptor);
        bytes.extend_from_slice(&self.sectors_per_fat.to_le_bytes());
        bytes
    }

    /// Instantiate a BIOS Parameter Block from a given disk type and OS combination.
    /// This only works for floppies. Hard disks get a similar function based on their
    /// geometry and OS. Floppies have a fixed, known geometry making this interface a
    /// more logical choice for them.
    pub fn from_floppy(disktype: &DiskType) -> Result<Self, OsError> {
        match disktype {
            DiskType::F525_160 => Ok(BPB {
                bytes_per_sector: 512,
                sectors_per_cluster: 1,
                reserved_sectors: 1,
                fat_copies: 2,
                rootdir_entries: 64,
                sector_count: 320,
                media_descriptor: 0xFE,
                sectors_per_fat: 1,
            }),
            DiskType::F525_180 => Ok(BPB {
                bytes_per_sector: 512,
                sectors_per_cluster: 1,
                reserved_sectors: 1,
                fat_copies: 2,
                rootdir_entries: 64,
                sector_count: 360,
                media_descriptor: 0xFC,
                sectors_per_fat: 2,
            }),
            DiskType::F525_320 => Ok(BPB {
                bytes_per_sector: 512,
                sectors_per_cluster: 2,
                reserved_sectors: 1,
                fat_copies: 2,
                rootdir_entries: 112,
                sector_count: 640,
                media_descriptor: 0xFF,
                sectors_per_fat: 2,
            }),
            DiskType::F525_360 => Ok(BPB {
                bytes_per_sector: 512,
                sectors_per_cluster: 2,
                reserved_sectors: 1,
                fat_copies: 2,
                rootdir_entries: 112,
                sector_count: 720,
                media_descriptor: 0xFD,
                sectors_per_fat: 2,
            }),
            _ => Err(OsError::NotAFloppy),
        }
    }
}
