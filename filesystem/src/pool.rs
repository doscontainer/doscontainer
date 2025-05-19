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
    /// Adds a new directory entry to the file system.
    ///
    /// This function validates the following before inserting the entry:
    /// - The entry must have a parent.
    /// - The parent must exist in the file system.
    /// - The parent must be a directory (not a file).
    /// - No other entry with the same name already exists under the same parent.
    ///
    /// # Arguments
    ///
    /// * `entry` - The `DirEntry` to be added.
    ///
    /// # Returns
    ///
    /// * `Ok(())` if the entry was successfully added.
    /// * `Err(FileSystemError)` if:
    ///   - The entry has no parent.
    ///   - The parent cannot be found.
    ///   - The parent is not a directory.
    ///   - A duplicate entry with the same name already exists under the parent.
    ///
    /// # Errors
    ///
    /// Returns one of the following `FileSystemError` variants:
    /// - `CannotAddParentlessEntry` if the entry lacks a parent reference.
    /// - `ParentNotFound` if the referenced parent does not exist.
    /// - `EntryCannotHaveChildren` if the parent is not a directory.
    /// - `DuplicateEntry` if another entry with the same name exists under the same parent.
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

        if self
            .children(parent_entry)
            .iter()
            .any(|e| e.name() == entry.name())
        {
            return Err(FileSystemError::DuplicateEntry);
        }

        self.entries.push(entry);
        Ok(())
    }

    /// Return an entry by its Uuid
    ///
    /// # Arguments
    ///
    /// * `uuid` - A reference to the Uuid for the entry you're looking for.
    ///
    /// # Returns
    ///
    /// - `Option<&DirEntry>` an optional reference to a DirEntry.
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

    /// Returns a reference to the root entry (if any)
    ///
    /// This method traverses the pool to find the root entry and returns either a reference
    /// to it or None if the pool doesn't have a root entry.
    ///
    /// # Returns
    ///
    /// An Option<&DirEntry>
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
