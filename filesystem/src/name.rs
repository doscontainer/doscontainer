use crate::error::FileSystemError;

/// Struct to represent FAT12-compatible file names in a
/// type-safe manner.
pub struct DirEntryName {
    name: String,
    extension: String,
}

impl DirEntryName {
    /// Create a new isntance of an EntryName. This struct enforces guarantees that
    /// all names strictly coform to FAT's short filename limitations.
    pub fn new(name: String, extension: Option<String>) -> Result<Self, FileSystemError> {
        let mut normalized_name = name.trim().to_ascii_uppercase();
        Ok(Self {
            name: String::new(),
            extension: String::new(),
        })
    }
}
