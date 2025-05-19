use std::{path::Path, str::FromStr};

use uuid::Uuid;

use crate::{direntry::DirEntry, error::FileSystemError, names::EntryName};

#[derive(Debug)]
pub struct Pool {
    entries: Vec<DirEntry>,
}

impl Default for Pool {
    /// Returns a new `Pool` containing only the root directory entry.
    fn default() -> Pool {
        Self {
            entries: vec![DirEntry::new_rootdir()],
        }
    }
}

impl Pool {
    pub fn add_entry(&mut self, entry: DirEntry) -> Result<(), FileSystemError> {
        let parent_uuid = entry
            .parent()
            .ok_or(FileSystemError::CannotAddParentlessEntry)?;

        let parent_entry = self
            .entry(parent_uuid)
            .ok_or(FileSystemError::ParentNotFound)?;

        if !parent_entry.is_directory() {
            return Err(FileSystemError::EntryCannotHaveChildren);
        }
        self.entries.push(entry);
        Ok(())
    }

    pub fn entry(&self, uuid: &Uuid) -> Option<&DirEntry> {
        self.entries.iter().find(|entry| entry.uuid() == uuid)
    }

    /// Finds a directory entry by its name within the children of a given parent directory.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the entry to find (as a string slice).
    /// * `parent` - A reference to the parent directory entry whose children will be searched.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing:
    /// - `Ok(Some(&DirEntry))` if an entry with the specified name exists among the parent's children.
    /// - `Ok(None)` if no matching entry is found.
    /// - `Err(FileSystemError)` if the provided name is invalid or cannot be parsed into an `EntryName`.
    ///
    /// # Errors
    ///
    /// This function returns an error if `name` is not a valid entry name as defined by `EntryName::from_str`.
    pub fn entry_by_name(
        &self,
        name: &str,
        parent: &DirEntry,
    ) -> Result<Option<&DirEntry>, FileSystemError> {
        let entry_name = EntryName::from_str(name)?;
        let children = self.children(parent);
        let entry = children
            .iter()
            .find(|entry| entry.name() == Some(&entry_name))
            .copied();
        Ok(entry)
    }

    /// Returns all directory entries that are direct children of the given parent entry.
    ///
    /// This method filters the internal list of directory entries and returns those whose
    /// parent UUID matches the UUID of the provided `parent`. If the parent has no children,
    /// an empty vector is returned.
    ///
    /// # Arguments
    ///
    /// * `parent` - A reference to the `DirEntry` whose children you want to retrieve.
    ///
    /// # Returns
    ///
    /// A `Vec` of references to `DirEntry` instances that are children of the given parent.
    pub fn children(&self, parent: &DirEntry) -> Vec<&DirEntry> {
        let parent_uuid = parent.uuid();
        self.entries
            .iter()
            .filter(|entry| entry.parent() == Some(parent_uuid))
            .collect()
    }

    pub fn root_entry(&self) -> Option<&DirEntry> {
        self.entries.iter().find(|entry| entry.is_root())
    }

    pub fn entry_by_path(&self, path: &Path) -> Option<&DirEntry> {
        if let Some(mut current) = self.root_entry() {
            for component in path.components() {}
            None
        } else {
            None
        }
    }
}
