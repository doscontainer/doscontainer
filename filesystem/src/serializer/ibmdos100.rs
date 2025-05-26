use chrono::{Datelike, NaiveDateTime, Timelike};

use crate::{
    allocationtable::{AllocationTable, ClusterValue},
    direntry::DirEntry,
    error::FileSystemError,
    names::EntryName,
    pool::Pool,
};

use super::{DirEntrySerializer, DirectorySerializer, Fat12Serializer, NameSerializer};

#[allow(dead_code)]
pub struct IbmDos100 {}

impl IbmDos100 {
    pub fn encode_time(dt: NaiveDateTime) -> u16 {
        let hour = dt.hour();
        let minute = dt.minute();
        let second = dt.second() / 2;
        ((hour as u16) << 11) | ((minute as u16) << 5) | (second as u16)
    }

    pub fn encode_date(dt: NaiveDateTime) -> u16 {
        let year = dt.year().clamp(1980, 2107) - 1980;
        let month = dt.month();
        let day = dt.day();
        ((year as u16) << 9) | ((month as u16) << 5) | (day as u16)
    }
}

impl DirEntrySerializer for IbmDos100 {
    fn serialize_direntry(entry: &DirEntry) -> Result<Vec<u8>, FileSystemError> {
        let mut buf = [0u8; 32];

        // Name + extension
        let name_bytes = match &entry.name() {
            Some(name) => IbmDos100::serialize_entryname(name)?,
            None => return Err(FileSystemError::EmptyFileName),
        };
        buf[0..11].copy_from_slice(&name_bytes);

        // Attributes
        // Intercept regular files with the Archive attribute here for PC-DOS 1.00
        // and set to 0x00 (no attribute set).
        if entry.attributes().as_byte() == 0x20 {
            buf[11] = 0x00;
        } else {
            buf[11] = entry.attributes().as_byte();
        }

        // 22–23: creation time
        let time = Self::encode_time(entry.creation_time());
        buf[22..24].copy_from_slice(&time.to_le_bytes());

        // 24–25: creation date
        let date = Self::encode_date(entry.creation_time());
        buf[24..26].copy_from_slice(&date.to_le_bytes());

        // Start cluster (0x1A–0x1B)
        let start_cluster = match entry.start_cluster() {
            Some(cluster) if cluster <= 0xFFF => cluster as u16,
            Some(_) => return Err(FileSystemError::ClusterOutOfBounds),
            None => 0,
        };
        buf[26..28].copy_from_slice(&start_cluster.to_le_bytes());

        // File size (0x1C–0x1F)
        if entry.file_size() > u32::MAX as usize {
            return Err(FileSystemError::FileTooLarge);
        }
        buf[28..32].copy_from_slice(&(entry.file_size() as u32).to_le_bytes());

        Ok(buf.to_vec())
    }
}

impl DirectorySerializer for IbmDos100 {
    fn serialize_directory(pool: &Pool, directory: &DirEntry) -> Result<Vec<u8>, FileSystemError> {
        let mut bytes: Vec<u8> = Vec::new();
        let children: Vec<_> = pool
            .iter()
            .filter(|entry| entry.parent() == Some(directory.uuid()))
            .collect();

        for child in &children {
            let child_bytes = <IbmDos100 as DirEntrySerializer>::serialize_direntry(child)?;
            bytes.extend(child_bytes);
        }

        if directory.is_root() {
            let placeholder_bytes: Vec<u8> = vec![
                0xE5, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6,
                0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6, 0xF6,
                0xF6, 0xF6, 0xF6, 0xF6,
            ];

            if children.len() < 64 {
                let placeholders_needed = 64 - children.len();
                for _ in 0..placeholders_needed {
                    bytes.extend(&placeholder_bytes);
                }
            }
        }
        Ok(bytes)
    }
}

impl Fat12Serializer for IbmDos100 {
    fn serialize_fat12(fat: &AllocationTable) -> Result<Vec<u8>, FileSystemError> {
        const FAT12_MASK: u16 = 0x0FFF;
        let clusters = fat.clusters();

        let mut fat_entries: Vec<u16> = Vec::new();

        // Set cluster 0 explicitly as media descriptor 0xFE
        fat_entries.push(0xFFE);

        // Set cluster 1 explicitly as EndOfChain
        fat_entries.push(0xFFF);

        // Serialize clusters from 2 to max
        let max_cluster = *clusters.keys().max().unwrap_or(&1); // at least 1 because we pushed two entries

        for i in 2..=max_cluster {
            let entry = match clusters.get(&i) {
                Some(ClusterValue::Next(n)) => {
                    if *n > FAT12_MASK as usize {
                        return Err(FileSystemError::ClusterOutOfBounds);
                    }
                    *n as u16
                }
                Some(ClusterValue::EndOfChain) => 0xFFF,
                Some(ClusterValue::Free) | None => 0x000,
                Some(ClusterValue::Bad) => 0xFF7,
                Some(ClusterValue::Reserved) => 0xFF0, // or any valid reserved value
            };
            fat_entries.push(entry);
        }

        let mut bytes = Vec::with_capacity((fat_entries.len() * 3).div_ceil(2));
        let mut i = 0;

        while i + 1 < fat_entries.len() {
            let a = fat_entries[i];
            let b = fat_entries[i + 1];

            bytes.push((a & 0xFF) as u8); // Low 8 bits of a

            let middle_byte = (((a >> 8) as u8) & 0x0F) | (((b as u8 & 0x0F) << 4) & 0xF0);
            //let middle_byte = (((a >> 8) as u8) & 0x0F) | (((b << 4) as u8) & 0xF0);
            bytes.push(middle_byte); // High 4 bits of a + low 4 bits of b

            bytes.push(u8::try_from(b >> 4).map_err(|_| FileSystemError::RangeLostInTruncation)?); // High 8 bits of b

            i += 2;
        }
        /*
                if i < fat_entries.len() {
                    let a = fat_entries[i];
                    bytes.push((a & 0xFF) as u8);
                    bytes.push(((a >> 8) as u8) & 0x0F);
                }
        */
        bytes.resize(fat.cluster_size(), 0);
        Ok(bytes)
    }
}

impl NameSerializer for IbmDos100 {
    fn serialize_entryname(name: &EntryName) -> Result<[u8; 11], FileSystemError> {
        let mut raw = [b' '; 11];

        let fname = name.filename.trim().to_uppercase();
        let ext = name.extension.trim().to_uppercase();

        if fname.len() > 8 || ext.len() > 3 {
            return Err(FileSystemError::FileNameTooLong);
        }

        for (i, c) in fname.bytes().take(8).enumerate() {
            raw[i] = c;
        }

        for (i, c) in ext.bytes().take(3).enumerate() {
            raw[8 + i] = c;
        }

        Ok(raw)
    }
}

#[cfg(test)]
mod tests {
    use chrono::{NaiveDate, NaiveTime};

    use super::*;
    use crate::names::EntryName;

    fn make_name(filename: &str, extension: &str) -> EntryName {
        EntryName {
            filename: filename.to_string(),
            extension: extension.to_string(),
        }
    }

    #[test]
    fn test_valid_short_name() {
        let name = make_name("FOO", "TXT");
        let raw = IbmDos100::serialize_entryname(&name).unwrap();
        assert_eq!(&raw, b"FOO     TXT");
    }

    #[test]
    fn test_exactly_8_plus_3() {
        let name = make_name("ABCDEFGH", "XYZ");
        let raw = IbmDos100::serialize_entryname(&name).unwrap();
        assert_eq!(&raw, b"ABCDEFGHXYZ");
    }

    #[test]
    fn test_padding_spaces() {
        let name = make_name("A", "B");
        let raw = IbmDos100::serialize_entryname(&name).unwrap();
        assert_eq!(&raw, b"A       B  ");
    }

    #[test]
    fn test_uppercase_conversion() {
        let name = make_name("foo", "txt");
        let raw = IbmDos100::serialize_entryname(&name).unwrap();
        assert_eq!(&raw, b"FOO     TXT");
    }

    #[test]
    fn test_too_long_filename() {
        let name = make_name("TOOLONGNAME", "OK");
        let err = IbmDos100::serialize_entryname(&name).unwrap_err();
        assert!(matches!(err, FileSystemError::FileNameTooLong));
    }

    #[test]
    fn test_too_long_extension() {
        let name = make_name("OK", "TOOLONG");
        let err = IbmDos100::serialize_entryname(&name).unwrap_err();
        assert!(matches!(err, FileSystemError::FileNameTooLong));
    }

    #[test]
    /// This test recreates a DirEntry for a system file named IBMBIO.COM, which was on the
    /// original release floppy for PC-DOS 1.00. It had a creation date/time of July 23 1981
    /// at midnight, which we'll replicate here. The start cluster on the original floppy was
    /// 2, which we reflect here, and the size of the original file was 1920 bytes
    /// which we're also reflecting in this test. The validation is for the serializer to create
    /// the exact 32 bytes that were in the original floppy's root directory for this file.
    fn serialize_ibmbio_com() {
        let date = NaiveDate::from_ymd_opt(1981, 7, 23).unwrap();
        let time = NaiveTime::from_hms_opt(0, 0, 0).unwrap();
        let datetime = NaiveDateTime::new(date, time);
        let mut ibmbio_com = DirEntry::new_sysfile("IBMBIO.COM").unwrap();
        ibmbio_com.set_creation_time(datetime);
        ibmbio_com.set_start_cluster(2); // Mkfile does this for us automatically
        ibmbio_com.set_filesize(1920);
        assert_eq!(
            IbmDos100::serialize_direntry(&ibmbio_com).unwrap(),
            vec![
                0x49, 0x42, 0x4d, 0x42, 0x49, 0x4f, 0x20, 0x20, 0x43, 0x4F, 0x4D, 0x06, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xF7, 0x02, 0x02, 0x00,
                0x80, 0x07, 0x00, 0x00
            ]
        );
    }
}
