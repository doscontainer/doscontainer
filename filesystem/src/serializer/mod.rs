use crate::{
    allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError, names::EntryName,
};

pub mod ibmdos100;

#[allow(dead_code)]
pub trait DirEntrySerializer {
    fn serialize_direntry(entry: &DirEntry) -> Result<Vec<u8>, FileSystemError>;
}

#[allow(dead_code)]
pub trait Fat12Serializer {
    fn serialize_fat12(fat: &AllocationTable) -> Result<Vec<u8>, FileSystemError>;
}

pub trait NameSerializer {
    fn serialize_entryname(name: &EntryName) -> Result<[u8; 11], FileSystemError>;
}
