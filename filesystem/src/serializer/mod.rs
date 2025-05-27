use crate::{
    allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError, names::EntryName,
    pool::Pool,
};

pub mod ibmdos100;

pub trait DirEntrySerializer {
    fn serialize_direntry(entry: &DirEntry) -> Result<Vec<u8>, FileSystemError>;
}

/// Takes a reference DirEntry that's a directory itself. Synthesizes the . and .. entries
/// if it's not the rootdir, then serializes all directly descending DirEntries into their
/// binary representation. Pads the total up to a the required number of entries if we're dealing
/// with a fixed rootdir allocation on IBM hardware.
pub trait DirectorySerializer {
    fn serialize_directory(fat: &Pool, directory: &DirEntry) -> Result<Vec<u8>, FileSystemError>;
}

pub trait Fat12Serializer {
    fn serialize_fat12(fat: &AllocationTable) -> Result<Vec<u8>, FileSystemError>;
}

pub trait NameSerializer {
    fn serialize_entryname(name: &EntryName) -> Result<[u8; 11], FileSystemError>;
}
