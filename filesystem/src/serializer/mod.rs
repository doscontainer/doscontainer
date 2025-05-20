use crate::{allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError};

pub mod ibmdos100;

pub trait DirEntrySerializer {
    fn serialize_direntry(entry: &DirEntry) -> Result<Vec<u8>, FileSystemError>;
}
pub trait Fat12Serializer {
    fn serialize_fat12(fat: &AllocationTable) -> Result<Vec<u8>, FileSystemError>;
}