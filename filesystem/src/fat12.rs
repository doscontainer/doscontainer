use std::path::Path;

use crate::{allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError, pool::Pool, FileSystem};

#[derive(Debug)]
pub struct Fat12 {
    allocation_table: AllocationTable,
    pool: Pool,
}

impl Default for Fat12 {
    fn default() -> Self {
        Fat12 {
            allocation_table: AllocationTable::default(),
            pool: Pool::default(),
        }
    }
}

impl Fat12 {
    /// Helper method: takes a path, returns the filename from it if it exists.
    fn get_filename(path: &Path) -> Result<Option<String>, FileSystemError> {
        let filename = path
            .components()
            .last()
            .and_then(|c| c.as_os_str().to_str());

        match filename {
            Some(name) => Ok(Some(name.to_string())),
            None => Ok(None),
        }
    }
}

impl FileSystem for Fat12 {
    fn mkfile(&mut self, path: &Path) -> Result<(), FileSystemError> {
        let filename = Self::get_filename(path)?.ok_or(FileSystemError::EmptyFileName)?;

        let mut entry = DirEntry::new_file(filename.as_str())?;

        let parent = self
            .pool
            .root_entry()
            .ok_or(FileSystemError::ParentNotFound)?;

        entry.set_parent(parent);
        self.pool.add_entry(entry)?;

        Ok(())
    }

    fn mkdir() {
        todo!()
    }

    fn rmfile() {
        todo!()
    }

    fn rmdir() {
        todo!()
    }

    fn is_file() {
        todo!()
    }

    fn is_directory() {
        todo!()
    }

    fn attrib() {
        todo!()
    }

    fn set_attrib() {
        todo!()
    }
}
