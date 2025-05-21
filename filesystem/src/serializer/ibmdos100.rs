use crate::{
    allocationtable::{AllocationTable, ClusterValue},
    error::FileSystemError,
};

use super::Fat12Serializer;

#[allow(dead_code)]
pub struct IbmDos100 {}

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
                Some(ClusterValue::Free) => 0x000,
                Some(ClusterValue::Bad) => 0xFF7,
                Some(ClusterValue::Reserved) => 0xFF0, // or any valid reserved value
                None => 0x000,                         // default to Free
            };
            fat_entries.push(entry);
        }

        let mut bytes = Vec::with_capacity((fat_entries.len() * 3 + 1) / 2);
        let mut i = 0;

        while i + 1 < fat_entries.len() {
            let a = fat_entries[i];
            let b = fat_entries[i + 1];

            bytes.push((a & 0xFF) as u8); // Low 8 bits of a
            bytes.push(((a >> 8) as u8 & 0x0F) | ((b << 4) as u8 & 0xF0)); // High 4 bits of a + low 4 bits of b
            bytes.push((b >> 4) as u8); // High 8 bits of b

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
