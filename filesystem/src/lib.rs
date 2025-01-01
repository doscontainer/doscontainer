use std::path::Path;

use self::error::FileSystemError;

mod allocationtable;
mod attributes;
mod cluster;
mod direntry;
mod error;
mod pool;

// Cluster index into the FAT
pub type ClusterIndex = usize;

pub enum FileType {
    RegularFile,
    SystemFile,
}
pub trait FileSystem {
    fn mkfile<P: AsRef<Path>,D: AsRef<[u8]>>(&mut self, path: P, data: D, filetype: FileType) -> Result<Vec<ClusterIndex>, FileSystemError>;
    fn mkdir<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<ClusterIndex>, FileSystemError>;
}