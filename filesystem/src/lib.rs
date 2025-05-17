use std::path::Path;

use allocationtable::AllocationTable;

use self::error::FileSystemError;

mod allocationtable;
mod attributes;
mod cluster;
mod direntry;
mod error;
pub mod fat12;
mod pool;

// Cluster index into the FAT
pub type ClusterIndex = usize;

pub enum FileType {
    RegularFile,
    SystemFile,
}

/// A trait representing a file system, providing methods for managing files and directories,
/// as well as interacting with the allocation table, which is responsible for keeping track
/// of the disk's clusters and free space.
///
/// Implementing this trait allows for the creation of files and directories, as well as
/// manipulating the underlying allocation table that defines the cluster mappings and usage
/// in the file system.
pub trait FileSystem {
    /// Retrieves an immutable reference to the allocation table.
    ///
    /// This method allows access to the allocation table, which maps clusters on the disk.
    /// The allocation table tracks free and used clusters, and is integral to managing the
    /// physical layout of data on the disk.
    ///
    /// # Returns
    /// * A reference to the allocation table.
    fn allocation_table(&self) -> &AllocationTable;

    /// Retrieves a mutable reference to the allocation table.
    ///
    /// This method allows modifying the allocation table, such as allocating or freeing clusters
    /// as files and directories are created or deleted. Modifications to the allocation table
    /// directly affect the physical layout of data on the disk.
    ///
    /// # Returns
    /// * A mutable reference to the allocation table.
    fn allocation_table_mut(&mut self) -> &mut AllocationTable;

    /// Creates a new file in the file system at the specified path with the given size and file type.
    ///
    /// The file will be created by allocating the necessary clusters on the disk, and its
    /// path will be added to the file system's directory structure.
    ///
    /// # Parameters
    /// * `path`: The path to the file to be created. It must implement the `AsRef<Path>` trait.
    /// * `size`: The size of the file to be created in bytes.
    /// * `filetype`: The type of the file being created, indicating whether it is a regular file
    ///   or a system file.
    ///
    /// # Returns
    /// * A `Result` containing a vector of cluster indices allocated to the file on success,
    ///   or a `FileSystemError` on failure.
    // fn mkfile<P: AsRef<Path>>(
    fn mkfile(
        &mut self,
        path: &Path,
        data: Vec<u8>,
        filetype: FileType,
    ) -> Result<Vec<ClusterIndex>, FileSystemError>;

    /// Creates a new directory in the file system at the specified path.
    ///
    /// This method will allocate the necessary clusters for the directory and update the
    /// directory structure to include the new directory at the specified path.
    ///
    /// # Parameters
    /// * `path`: The path to the directory to be created. It must implement the `AsRef<Path>` trait.
    ///
    /// # Returns
    /// * A `Result` containing a vector of cluster indices allocated to the directory on success,
    ///   or a `FileSystemError` on failure.
    fn mkdir(&mut self, path: &Path) -> Result<Vec<ClusterIndex>, FileSystemError>;
}
