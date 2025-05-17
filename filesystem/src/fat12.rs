use std::path::Path;

use crate::{
    allocationtable::AllocationTable,
    direntry::{DirEntry, DirEntryType},
    error::FileSystemError,
    pool::Pool,
    ClusterIndex, FileSystem, FileType,
};
use disk::{disktype::DiskType, Disk};
use operatingsystem::{OperatingSystem, OsShortName};

pub struct Fat12 {
    allocation_table: AllocationTable,
    pool: Pool,
    os: OperatingSystem,
}

impl FileSystem for Fat12 {
    fn mkfile(
        &mut self,
        path: &Path,
        data: Vec<u8>,
        filetype: FileType,
    ) -> Result<Vec<ClusterIndex>, FileSystemError> {
        // Figure out how many clusters we're going to need for the file we're making
        let size = data.len();
        let cluster_count = size.div_ceil(self.allocation_table.cluster_size());

        // Prep the list of clusters by reserving them in the AllocationTable
        let mut reserved_clusters = Vec::with_capacity(cluster_count);
        for _ in 0..cluster_count {
            match self.allocation_table.reserve_cluster() {
                Ok(index) => reserved_clusters.push(index),
                Err(e) => return Err(e),
            }
        }

        // Create the directory entry as needed by filetype
        match filetype {
            FileType::RegularFile => {
                let mut entry = DirEntry::new(DirEntryType::File);
                entry.set_allocated_clusters(&reserved_clusters);
                todo!();
            }
            FileType::SystemFile => {
                let mut entry = DirEntry::new(DirEntryType::SysFile);
                entry.set_allocated_clusters(&reserved_clusters);
                todo!();
            }
        }

        // Write file's bytes to the disk
    }

    fn mkdir(
        &mut self,
        path: &Path,
    ) -> Result<Vec<crate::ClusterIndex>, crate::error::FileSystemError> {
        todo!()
    }

    /// Returns a reference to the `AllocationTable` of the filesystem.
    ///
    /// This method provides access to the allocation table, allowing read-only access
    /// to the table. It can be used to inspect the allocation of clusters in the
    /// FAT12 file system.
    ///
    /// # Returns
    /// * `&AllocationTable` - A reference to the `AllocationTable` instance associated
    ///   with this `Fat12` filesystem.
    fn allocation_table(&self) -> &AllocationTable {
        &self.allocation_table
    }

    /// Returns a mutable reference to the `AllocationTable` of the filesystem.
    ///
    /// This method allows mutable access to the allocation table, enabling modifications
    /// to the allocation of clusters within the FAT12 filesystem. It is typically used
    /// for operations that modify the allocation table, such as allocating new clusters
    /// or updating existing ones.
    ///
    /// # Returns
    /// * `&mut AllocationTable` - A mutable reference to the `AllocationTable` instance
    ///   associated with this `Fat12` filesystem, allowing modifications to the table.
    fn allocation_table_mut(&mut self) -> &mut AllocationTable {
        &mut self.allocation_table
    }
}

impl Fat12 {
    pub fn new(os: OperatingSystem, disk: &dyn Disk) -> Result<Self, FileSystemError> {
        let cluster_size = match disk.disktype() {
            DiskType::F525_160 => 1,
            DiskType::F525_180 => 1,
            DiskType::F525_320 => 2,
            DiskType::F525_360 => 2,
            _ => return Err(FileSystemError::InvalidDiskType),
        };

        // Cluster count comes from Microsoft docs. Error for not-yet supported types. Numbers are present
        // in the code but commented out to save the effort of looking them up again. Enable as needed when
        // supporting newer DOS versions.
        let cluster_count = match disk.disktype() {
            DiskType::F525_160 => 340,
            DiskType::F525_180 => 351,
            DiskType::F525_320 => 315,
            DiskType::F525_360 => 354,
            // DiskType::F525_1200 => 2371,
            // DiskType::F35_720 => 713,
            // DiskType::F35_1440 => 2847,
            // DiskType::F35_2880 => 2863,
            _ => return Err(FileSystemError::InvalidDiskType),
        };
        let mut filesystem = Fat12 {
            allocation_table: AllocationTable::new(cluster_count, cluster_size)?,
            pool: Pool::new()?,
            os: os,
        };

        // Different OS'es do different things with the first clusters in the allocation table
        match filesystem.os.shortname() {
            OsShortName::IBMDOS100 => {
                // Allocate the first two clusters as they are in PC-DOS 1.00
                filesystem
                    .allocation_table_mut()
                    .allocate_cluster(0, 0xFFE)?;
                filesystem
                    .allocation_table_mut()
                    .allocate_cluster(1, 0xFFF)?;
            }
            OsShortName::IBMDOS110 => {
                filesystem
                    .allocation_table_mut()
                    .allocate_cluster(0, 0xFFF)?;
                filesystem
                    .allocation_table_mut()
                    .allocate_cluster(1, 0xFFF)?;
            }
            OsShortName::IBMDOS200 => {
                // FAT ID depends on the media descriptor byte
                match disk.disktype() {
                    DiskType::F525_180 => filesystem
                        .allocation_table_mut()
                        .allocate_cluster(0, 0xFFC)?,
                    DiskType::F525_160 => filesystem
                        .allocation_table_mut()
                        .allocate_cluster(0, 0xFFE)?,
                    DiskType::F525_360 => filesystem
                        .allocation_table_mut()
                        .allocate_cluster(0, 0xFFD)?,
                    DiskType::F525_320 => filesystem
                        .allocation_table_mut()
                        .allocate_cluster(0, 0xFFF)?,
                    _ => return Err(FileSystemError::UnsupportedDiskType),
                }
                filesystem
                    .allocation_table_mut()
                    .allocate_cluster(1, 0xFFF)?;
            }
            _ => (),
        }

        // Only then we return the filesystem for further use.
        Ok(filesystem)
    }
}
