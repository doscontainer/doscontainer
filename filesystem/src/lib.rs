use std::path::Path;

use error::FileSystemError;

mod allocationtable;
mod attributes;
mod direntry;
mod error;
mod fat12;
mod names;
mod pool;

// Cluster index into the FAT
pub type ClusterIndex = usize;

#[cfg(test)]
mod tests;

pub trait FileSystem {
    /// Create a new file
    fn mkfile(&mut self, path: &str, data: &[u8]) -> Result<(), FileSystemError>;

    /// Create a directory
    fn mkdir();

    /// Remove a file
    fn rmfile();

    /// Remove a directory
    fn rmdir();

    /// Check if an entry is a file
    fn is_file();

    /// Check if an entry is a directory
    fn is_directory();

    /// Get the file system attributes from an entry
    fn attrib();

    /// Set the file system attributes on an entry
    fn set_attrib();
}
