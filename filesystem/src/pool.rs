use uuid::Uuid;

use crate::{direntry::DirEntry, error::FileSystemError};

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

        if !parent_entry.can_be_parent() {
            return Err(FileSystemError::EntryCannotHaveChildren);
        }

        self.entries.push(entry);
        Ok(())
    }

    pub fn entry(&self, uuid: &Uuid) -> Option<&DirEntry> {
        self.entries.iter().find(|entry| entry.uuid() == uuid)
    }
}
