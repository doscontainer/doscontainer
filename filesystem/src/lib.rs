use error::FileSystemError;

mod allocationtable;
mod attributes;
mod bpb;
mod direntry;
mod error;
mod fat12;
mod names;
mod pool;
mod serializer;

// Cluster index into the FAT
pub type ClusterIndex = usize;

#[cfg(test)]
mod tests;

pub trait FileSystem {
    /// Create a new file
    fn mkfile(&mut self, path: &str, filesize: usize) -> Result<(), FileSystemError>;

    /// Create a directory
    fn mkdir(&mut self, path: &str, entries_count: usize) -> Result<(), FileSystemError>;
}
