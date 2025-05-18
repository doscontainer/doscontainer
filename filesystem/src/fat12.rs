use std::path::Path;

use crate::{
    attributes::Attributes, direntry::DirEntry, error::FileSystemError, names::EntryName,
    pool::Pool, FileSystem,
};

pub struct Fat12 {
    pool: Pool,
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
            None => Ok(None)
        }
    }
}

impl FileSystem for Fat12 {
    fn mkfile(&mut self, path: &Path) -> Result<(), FileSystemError> {
        if let Some(filename) = Self::get_filename(path)? {
            Ok(())
        } else {
            return Err(FileSystemError::EmptyFileName)
        }
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
