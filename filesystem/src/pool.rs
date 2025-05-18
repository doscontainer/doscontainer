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