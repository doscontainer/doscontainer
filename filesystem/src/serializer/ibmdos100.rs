use crate::{
    allocationtable::{AllocationTable, ClusterValue},
    direntry::DirEntry,
    error::FileSystemError,
    names::EntryName,
};

use super::{DirEntrySerializer, Fat12Serializer, NameSerializer};

#[allow(dead_code)]
pub struct IbmDos100 {}

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
        buf[11] = entry.attributes().as_byte(); // Make sure this method exists

        // Reserved (0x0C–0x15): leave as zero

        // Time/date (0x16–0x19): leave as zero

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

impl Fat12Serializer for IbmDos100 {
    fn serialize_fat12(fat: &AllocationTable) -> Result<Vec<u8>, FileSystemError> {
        const FAT12_MASK: u16 = 0x0FFF;
        let clusters = fat.clusters();

        let mut fat_entries: Vec<u16> = Vec::new();

        // Set cluster 0 explicitly as media descriptor 0xFE (0xFFE 12-bit value)
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

            let middle_byte = (((a >> 8) as u8) & 0x0F) | (((b << 4) as u8) & 0xF0);
            bytes.push(middle_byte); // High 4 bits of a + low 4 bits of b

            bytes.push(u8::try_from(b >> 4).map_err(|_| FileSystemError::RangeLostInTruncation)?); // High 8 bits of b

            i += 2;
        }

        if i < fat_entries.len() {
            let a = fat_entries[i];
            bytes.push((a & 0xFF) as u8);
            bytes.push(((a >> 8) as u8) & 0x0F);
        }

        Ok(bytes)
    }
}

impl NameSerializer for IbmDos100 {
    fn serialize_entryname(name: &EntryName) -> Result<[u8; 11], FileSystemError> {
        let mut raw = [b' '; 11];

        let fname = name.filename.to_uppercase();
        let ext = name.extension.to_uppercase();

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
