use uuid::Uuid;

use crate::{
    direntry::{DirEntry, DirEntryType},
    error::FileSystemError,
};

pub struct Pool {
    entries: Vec<DirEntry>,
}

impl Pool {
    /// A new pool starts out with a single nameless, parentless Directory entry for the rootdir.
    pub fn new() -> Result<Self, FileSystemError> {
        let mut pool = Pool {
            entries: Vec::new(),
        };
        let rootdir = DirEntry::default();
        pool.add_entry(rootdir)?;
        Ok(pool)
    }

    /// Adds a directory entry to the pool.
    ///
    /// # Parameters
    /// - `entry`: The [`DirEntry`] to be added to the pool.
    ///
    /// # Returns
    /// - `Ok(())` if the entry is successfully added.
    /// - `Err(FileSystemError::InvalidEntryType)` if the pool is empty and the
    ///   provided entry is not a directory.
    ///
    /// # Special Behavior
    /// - If the pool is empty, only a root directory entry (of type [`DirEntryType::Directory`])
    ///   can be added as the first entry. Adding any other entry type will result in an error.
    pub fn add_entry(&mut self, entry: DirEntry) -> Result<(), FileSystemError> {
        // Special case: pool is empty, we only allow adding a root directory entry.
        if self.entries.is_empty() {
            // First entry must be a directory
            if entry.entry_type() != DirEntryType::Directory {
                return Err(FileSystemError::InvalidEntryType);
            }
            // First entry must not have a parent
            if entry.parent().is_some() {
                return Err(FileSystemError::InvalidEntryType);
            }
        }

        // Ensure the entry's ID is unique in the pool
        if self.entries.iter().any(|e| e.id() == entry.id()) {
            return Err(FileSystemError::DuplicateEntry);
        }

        // Ensure the entry's parent is present in the pool and is of a type that's allowed to have children
        if let Some(parent_id) = entry.parent() {
            match self.entry_by_id(parent_id) {
                Some(parent_entry) => {
                    // Ensure the parent is of a type that's allowed to have children
                    if parent_entry.entry_type() != DirEntryType::Directory {
                        return Err(FileSystemError::EntryCanNotHaveChildren);
                    }
                }
                None => {
                    return Err(FileSystemError::EntryDoesNotExist);
                }
            }
        }

        // Add the entry to the pool
        self.entries.push(entry);
        Ok(())
    }

    pub fn entry_by_id(&self, id: Uuid) -> Option<&DirEntry> {
        self.entries.iter().find(|entry| entry.id() == id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pool_new_initializes_with_rootdir() {
        let pool = Pool::new().unwrap();
        assert_eq!(
            pool.entries.len(),
            1,
            "Pool should start with a single root directory entry."
        );
        assert_eq!(
            pool.entries[0].entry_type(),
            DirEntryType::Directory,
            "The initial entry should be of type Directory."
        );
    }

    #[test]
    fn test_add_entry_to_empty_pool() {
        let mut pool = Pool {
            entries: Vec::new(),
        };

        let root_dir = DirEntry::new(DirEntryType::Directory);
        assert!(
            pool.add_entry(root_dir).is_ok(),
            "Should allow adding a root directory to an empty pool."
        );
    }

    #[test]
    fn test_add_non_directory_to_empty_pool() {
        let mut pool = Pool {
            entries: Vec::new(),
        };

        let file_entry = DirEntry::new(DirEntryType::File);
        let result = pool.add_entry(file_entry);
        assert!(
            matches!(result, Err(FileSystemError::InvalidEntryType)),
            "Adding a non-directory entry to an empty pool should return InvalidEntryType error."
        );
    }

    #[test]
    fn test_add_entry_to_non_empty_pool() {
        let mut pool = Pool::new().unwrap();

        let file_entry = DirEntry::new(DirEntryType::File);
        assert!(
            pool.add_entry(file_entry).is_ok(),
            "Should allow adding a file entry to a non-empty pool."
        );
        assert_eq!(
            pool.entries.len(),
            2,
            "Pool should contain two entries after adding a file entry."
        );
    }

    #[test]
    fn test_multiple_entries_in_pool() {
        let mut pool = Pool::new().unwrap();

        let subdir = DirEntry::new(DirEntryType::Directory);
        let file_entry = DirEntry::new(DirEntryType::File);

        assert!(
            pool.add_entry(subdir).is_ok(),
            "Should allow adding a directory entry to a non-empty pool."
        );
        assert!(
            pool.add_entry(file_entry).is_ok(),
            "Should allow adding a file entry to a non-empty pool."
        );

        assert_eq!(
            pool.entries.len(),
            3,
            "Pool should contain three entries after adding two more."
        );
        assert_eq!(
            pool.entries[1].entry_type(),
            DirEntryType::Directory,
            "Second entry should be of type Directory."
        );
        assert_eq!(
            pool.entries[2].entry_type(),
            DirEntryType::File,
            "Third entry should be of type File."
        );
    }
}
