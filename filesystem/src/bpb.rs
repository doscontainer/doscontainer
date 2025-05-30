use chrono::{Datelike, Local, Timelike};
use common::storage::FloppyType;
use disk::sectorsize::SectorSize;

use crate::error::FileSystemError;

#[derive(Debug)]
pub struct BiosParameterBlock {
    bytes_per_sector: usize,
    sectors_per_cluster: usize,
    reserved_sectors: usize,
    fat_count: usize,
    root_dir_entries: usize,
    logical_sector_count: usize,
    media_descriptor: u8,
    sectors_per_fat: usize,
    // Added with DOS 3.0
    /* sectors_per_track: usize,
    number_of_heads: usize,
    hidden_sectors: usize,
    // Added with DOS 3.2
    physical_sector_count: usize,
    // Added with DOS 3.4
    physical_drive_number: usize,
    flags: usize,
    extended_boot_signature: usize,
    volume_serial_number: u32,
    volume_label: String,
    filesystem_type: usize, */
}

impl Default for BiosParameterBlock {
    fn default() -> Self {
        // This is a 160KB floppy disk
        BiosParameterBlock::new(SectorSize::S512, 1, 1, 64, 320, 0xFE, 2)
    }
}

impl BiosParameterBlock {
    /// These values may not be correct. Only F525_160 is currently verified from actual systems.
    pub fn from_floppytype(floppy_type: &FloppyType) -> Self {
        match floppy_type {
            FloppyType::F525_160 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 1,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 64,
                logical_sector_count: 320,
                media_descriptor: 0xFE,
                sectors_per_fat: 1,
            },
            FloppyType::F525_180 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 1,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 64,
                logical_sector_count: 360,
                media_descriptor: 0xFC,
                sectors_per_fat: 1,
            },
            FloppyType::F525_320 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 2,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 112,
                logical_sector_count: 640,
                media_descriptor: 0xFF,
                sectors_per_fat: 2,
            },
            FloppyType::F525_360 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 2,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 112,
                logical_sector_count: 720,
                media_descriptor: 0xFD,
                sectors_per_fat: 2,
            },
            FloppyType::F525_1200 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 1,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 224,
                logical_sector_count: 2400,
                media_descriptor: 0xF9,
                sectors_per_fat: 7,
            },
            FloppyType::F35_720 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 2,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 112,
                logical_sector_count: 1440,
                media_descriptor: 0xF9,
                sectors_per_fat: 3,
            },
            FloppyType::F35_1440 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 1,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 224,
                logical_sector_count: 2880,
                media_descriptor: 0xF0,
                sectors_per_fat: 9,
            },
            FloppyType::F35_2880 => BiosParameterBlock {
                bytes_per_sector: 512,
                sectors_per_cluster: 2,
                reserved_sectors: 1,
                fat_count: 2,
                root_dir_entries: 240,
                logical_sector_count: 5760,
                media_descriptor: 0xF0,
                sectors_per_fat: 9,
            },
        }
    }

    pub fn new(
        sector_size: SectorSize,
        sectors_per_cluster: usize,
        reserved_sectors: usize,
        root_dir_entries: usize,
        sector_count: usize,
        media_descriptor: u8,
        sectors_per_fat: usize,
    ) -> Self {
        Self {
            bytes_per_sector: sector_size.as_usize(),
            sectors_per_cluster,
            reserved_sectors,
            fat_count: 2,
            root_dir_entries,
            logical_sector_count: sector_count,
            media_descriptor,
            sectors_per_fat,
            /* sectors_per_track: todo!(),
            number_of_heads: todo!(),
            hidden_sectors: todo!(),
            physical_sector_count: todo!(),
            physical_drive_number: todo!(),
            flags: todo!(),
            extended_boot_signature: todo!(),
            volume_serial_number: todo!(),
            volume_label: todo!(),
            filesystem_type: todo!(), */
        }
    }

    pub fn set_sectors_per_cluster(&mut self, sector_count: usize) -> Result<(), FileSystemError> {
        match sector_count {
            1 => self.sectors_per_cluster = 1,
            2 => self.sectors_per_cluster = 2,
            4 => self.sectors_per_cluster = 4,
            8 => self.sectors_per_cluster = 8,
            16 => self.sectors_per_cluster = 16,
            32 => self.sectors_per_cluster = 32,
            64 => self.sectors_per_cluster = 64,
            128 => self.sectors_per_cluster = 128,
            _ => return Err(FileSystemError::InvalidSectorsPerCluster),
        }
        Ok(())
    }

    pub fn generate_volume_serial_number() -> u32 {
        let now = Local::now();

        let year = (now.year().max(1980) - 1980) as u16; // cast to u16
        let month = now.month() as u16;
        let day = now.day() as u16;
        let hour = now.hour() as u16;
        let minute = now.minute() as u16;
        let second = (now.second() / 2) as u16; // DOS timestamps store seconds / 2

        let time_part: u16 = (hour << 11) | (minute << 5) | second;
        let date_part: u16 = (year << 9) | (month << 5) | day;

        (u32::from(time_part) << 16) | u32::from(date_part)
    }
}
pub enum MediaDescriptor {}
