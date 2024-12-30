use disk::disktype::DiskType;

use crate::error::OsError;

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
    /// Instantiate a BIOS Parameter Block from a given disk type and OS combination.
    /// This only works for floppies. Hard disks get a similar function based on their
    /// geometry and OS. Floppies have a fixed, known geometry making this interface a
    /// more logical choice for them. Since we're supporting a bazillion permutations here,
    /// we call out to private methods from this function to do the actual work.
    pub fn from_disktype(disktype: &DiskType) -> Result<Self, OsError> {
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
            _ => Err(OsError::UnsupportedDiskType),
        }
    }
}
