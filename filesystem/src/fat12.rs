use std::path::Path;

use chrono::NaiveDateTime;
use disk::{sectorsize::SectorSize, volume::Volume, Disk};
use operatingsystem::OperatingSystem;

use crate::{
    allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError, pool::Pool, serializer::{ibmdos100::IbmDos100, DirectorySerializer, Fat12Serializer}, ClusterIO, ClusterIndex, FileSystem
};

#[derive(Debug)]
pub struct Fat12<'a, D: Disk> {
    allocation_table: AllocationTable,
    pool: Pool,
    cluster_size: usize, // Cluster size in sectors
    cluster_count: usize,
    sector_size: SectorSize,
    volume: &'a mut Volume<'a, D>,
}

impl<'a, D: Disk> ClusterIO for Fat12<'a, D> {
    /// Writes the contents of a cluster at the given cluster index.
    ///
    /// Pads the input data with zeros if it is smaller than the expected cluster size.
    /// Returns an error if the data is too large to fit in a single cluster.
    ///
    /// # Parameters
    /// - `index`: The cluster index to write to. Must be ≥ 2.
    /// - `data`: The data to write. If shorter than the cluster size, it will be zero-padded.
    ///
    /// # Returns
    /// `Ok(())` if the cluster was written successfully, or an appropriate `FileSystemError`.
    fn write_cluster(&mut self, index: ClusterIndex, data: &[u8]) -> Result<(), FileSystemError> {
        let first_sector = self.cluster_to_sector(index);
        let sector_size_bytes = self.sector_size.as_usize();
        let sectors_per_cluster = self.cluster_size;

        let expected_len = sector_size_bytes * sectors_per_cluster;

        if data.len() > expected_len {
            return Err(FileSystemError::ClusterTooLarge);
        }

        // Allocate buffer with zeroed padding
        let mut buffer = vec![0u8; expected_len];
        buffer[..data.len()].copy_from_slice(data);

        for i in 0..sectors_per_cluster {
            let offset = i * sector_size_bytes;
            let sector_data = &buffer[offset..offset + sector_size_bytes];
            self.volume
                .write_sector(first_sector as u64 + i as u64, sector_data)
                .map_err(|_| FileSystemError::DiskError)?;
        }

        Ok(())
    }

    /// Converts a cluster index to the corresponding starting sector number.
    ///
    /// Cluster indices must start from 2, as per FAT12 conventions. This calculation
    /// assumes a contiguous layout of clusters following the data region start.
    ///
    /// # Parameters
    /// - `index`: The cluster index (must be ≥ 2).
    ///
    /// # Returns
    /// The first sector number corresponding to the start of the given cluster.
    fn cluster_to_sector(&self, index: ClusterIndex) -> usize {
        self.data_region_start() + ((index - 2) as usize * self.cluster_size as usize)
    }

    /// Returns the starting sector number of the data region.
    ///
    /// This implementation assumes a PC-DOS 1.00 layout and is hardcoded accordingly.
    /// Future versions should derive this from the actual BPB or filesystem metadata.
    ///
    /// # Returns
    /// The sector number where the first data cluster begins.
    fn data_region_start(&self) -> usize {
        7
    }
}

impl<'a, D: Disk> Fat12<'a, D> {
    pub fn new(
        sector_size: SectorSize,
        cluster_size: usize,
        cluster_count: usize,
        volume: &'a mut Volume<'a, D>,
    ) -> Result<Self, FileSystemError> {
        let filesystem = Fat12 {
            allocation_table: AllocationTable::default(),
            pool: Pool::default(),
            cluster_size,
            cluster_count,
            sector_size,
            volume,
        };
        Ok(filesystem)
    }

    /// THIS HAS TO GO!!
    pub fn write_crud(&mut self) {
        let os = operatingsystem::OperatingSystem::from_vendor_version("ibm", "1.00").unwrap();
        let fatbytes = IbmDos100::serialize_fat12(&self.allocation_table()).unwrap();
        let databytes = IbmDos100::serialize_directory(
            self.pool(),
            self.pool().root_entry().unwrap(),
        )
        .unwrap();
        self.volume.write_sector(0, os.bootsector()).unwrap();
        self.volume.write_sector(1, &fatbytes).unwrap();
        self.volume.write_sector(2, &fatbytes).unwrap();
        for (i, chunk) in databytes.chunks(512).enumerate() {
            self.volume.write_sector(3 + i as u64, chunk).unwrap();
        }
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

impl<'a, D: disk::Disk> FileSystem for Fat12<'a, D> {
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
        data: &[u8],
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

        let clusters = self.allocation_table.allocate_entry(data.len())?;
        entry.set_cluster_map(&clusters);
        entry.set_start_cluster(clusters[0]);
        entry.set_filesize(data.len());

        // Write data to each cluster
        let cluster_bytes = self.sector_size.as_usize() * self.cluster_size;
        for (i, &cluster) in clusters.iter().enumerate() {
            let start = i * cluster_bytes;
            let end = usize::min(start + cluster_bytes, data.len());
            self.write_cluster(cluster, &data[start..end])?;
        }

        self.pool.add_entry(entry)?;

        Ok(())
    }

    fn mksysfile(
        &mut self,
        path_str: &str,
        data: &[u8],
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

        let clusters = self.allocation_table.allocate_entry(data.len())?;
        entry.set_cluster_map(&clusters);
        entry.set_start_cluster(clusters[0]);
        entry.set_filesize(data.len());

        // Write data to each cluster
        let cluster_bytes = self.sector_size.as_usize() * self.cluster_size;
        for (i, &cluster) in clusters.iter().enumerate() {
            let start = i * cluster_bytes;
            let end = usize::min(start + cluster_bytes, data.len());
            self.write_cluster(cluster, &data[start..end])?;
        }

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
