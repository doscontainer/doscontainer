use chrono::{Datelike, Local, Timelike};

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
    sectors_per_track: usize,
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
    filesystem_type: usize,
}

impl BiosParameterBlock {
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
