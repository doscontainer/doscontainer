use std::path::Path;

use chrono::NaiveDateTime;

use crate::{
    allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError, pool::Pool,
    FileSystem,
};

#[derive(Debug)]
pub struct Fat12 {
    allocation_table: AllocationTable,
    pool: Pool,
    cluster_size: usize, // Cluster size in sectors
    cluster_count: usize,
    sector_size: usize,
}

impl Default for Fat12 {
    fn default() -> Self {
        Fat12 {
            allocation_table: AllocationTable::default(),
            pool: Pool::default(),
            cluster_size: 1,    // Size in sectors
            cluster_count: 313, // Number of clusters in the filesystem
            sector_size: 512,   // Sector size in bytes
        }
    }
}

impl Fat12 {
    pub fn new(
        sector_size: usize,
        cluster_size: usize,
        cluster_count: usize,
    ) -> Result<Self, FileSystemError> {
        let mut filesystem = Fat12::default();
        filesystem
            .allocation_table
            .set_cluster_count(cluster_count)?;
        filesystem.cluster_count = cluster_count;
        filesystem.cluster_size = cluster_size;
        filesystem.sector_size = sector_size;
        Ok(filesystem)
    }

    pub fn allocation_table(&self) -> &AllocationTable {
        &self.allocation_table
    }

    pub fn pool(&self) -> &Pool {
        &self.pool
    }

    /// Helper method: takes a path, returns the filename from it if it exists.
    fn get_filename(path: &Path) -> Option<String> {
        let filename = path
            .components()
            .next_back()
            .and_then(|c| c.as_os_str().to_str());

        match filename {
            Some(name) => Some(name.to_string()),
            None => None,
        }
    }
}

impl FileSystem for Fat12 {
    /// Creates a new file entry at the specified path.
    ///
    /// The path should include the filename. The file will be added
    /// under its parent directory if it exists in the pool.
    ///
    /// # Errors
    /// Returns `FileSystemError::EmptyFileName` if the filename is empty,
    /// or `FileSystemError::ParentNotFound` if the parent directory doesn't exist,
    /// or errors returned by `DirEntry::new_file` or `pool.add_entry`.
    fn mkfile(
        &mut self,
        path_str: &str,
        filesize: usize,
        creation_time: Option<NaiveDateTime>,
    ) -> Result<(), FileSystemError> {
        let path = Path::new(path_str);

        // Convert Option to Result here
        let filename = Self::get_filename(path).ok_or(FileSystemError::EmptyFileName)?;
        let parent_path = path.parent().ok_or(FileSystemError::ParentNotFound)?;
        let parent = self
            .pool
            .entry_by_path(parent_path)
            .ok_or(FileSystemError::ParentNotFound)?;

        let mut entry = DirEntry::new_file(filename.as_str())?;
        // If we're given a real creation time, use it. Otherwise it'll be the current host system clock.
        if let Some(time) = creation_time {
            entry.set_creation_time(time);
        }

        entry.set_parent(parent);

        let clusters = self.allocation_table.allocate_entry(filesize)?;
        entry.set_cluster_map(&clusters);
        entry.set_start_cluster(clusters[0]);
        entry.set_filesize(filesize);

        self.pool.add_entry(entry)?;

        Ok(())
    }

    fn mksysfile(
        &mut self,
        path_str: &str,
        filesize: usize,
        creation_time: Option<NaiveDateTime>,
    ) -> Result<(), FileSystemError> {
        let path = Path::new(path_str);

        // Convert Option to Result here
        let filename = Self::get_filename(path).ok_or(FileSystemError::EmptyFileName)?;
        let parent_path = path.parent().ok_or(FileSystemError::ParentNotFound)?;
        let parent = self
            .pool
            .entry_by_path(parent_path)
            .ok_or(FileSystemError::ParentNotFound)?;

        let mut entry = DirEntry::new_sysfile(filename.as_str())?;
        // If we're given a real creation time, use it. Otherwise it'll be the current host system clock.
        if let Some(time) = creation_time {
            entry.set_creation_time(time);
        }

        entry.set_parent(parent);

        let clusters = self.allocation_table.allocate_entry(filesize)?;
        entry.set_cluster_map(&clusters);
        entry.set_start_cluster(clusters[0]);
        entry.set_filesize(filesize);

        self.pool.add_entry(entry)?;

        Ok(())
    }

    fn mkdir(
        &mut self,
        path: &str,
        entries_count: usize,
        creation_time: Option<NaiveDateTime>,
    ) -> Result<(), FileSystemError> {
        let path = Path::new(path);

        const DIRENTRY_SIZE: usize = 32;
        const SYSTEM_ENTRIES: usize = 2;

        // The SYSTEM_ENTRIES are "." and "..". They don't exist in the in-memory model
        // but they will upon final allocation, so take them into account here to ensure
        // correct sizing calculations.
        let on_disk_size = (entries_count + SYSTEM_ENTRIES) * DIRENTRY_SIZE;

        let dirname = Self::get_filename(path).ok_or(FileSystemError::EmptyFileName)?;

        let mut entry = DirEntry::new_directory(dirname.as_str())?;
        // If we're given a real creation time, use it. Otherwise it'll be the current host system clock.
        if let Some(time) = creation_time {
            entry.set_creation_time(time);
        }

        // Get the parent directory path (if any)
        let parent_path = path.parent().ok_or(FileSystemError::ParentNotFound)?;

        // Find the parent entry in the pool
        if let Some(parent) = self.pool.entry_by_path(parent_path) {
            entry.set_parent(parent);

            // Allocate one cluster for the directory
            let clusters = self.allocation_table.allocate_entry(on_disk_size)?;
            entry.set_cluster_map(&clusters);
            entry.set_start_cluster(clusters[0]);
            entry.set_filesize(on_disk_size);

            // Add the directory entry to the pool
            self.pool.add_entry(entry)?;
            Ok(())
        } else {
            Err(FileSystemError::ParentNotFound)
        }
    }
}
