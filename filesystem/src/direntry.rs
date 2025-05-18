use std::str::FromStr;

use uuid::Uuid;

use crate::{
    attributes::{Attributes, AttributesPreset},
    error::FileSystemError,
    names::EntryName,
};

#[derive(Debug, PartialEq)]
pub struct DirEntry {
    uid: Uuid,
    attributes: Attributes,
    name: EntryName,
}

impl DirEntry {
    /// Create a regular file
    pub fn new_file(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::RegularFile)
    }

    /// Create a system file
    pub fn new_sysfile(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::SystemFile)
    }

    /// Create a new (sub)directory
    pub fn new_directory(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::Directory)
    }

    /// Create a new volume label entry
    pub fn new_volume_label(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::VolumeLabel)
    }

    /// Create a new placeholder record (this is an IBM-ism, see docs)
    pub fn new_placeholder(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::EmptyPlaceholder)
    }

    fn new_from_preset(name: &str, preset: AttributesPreset) -> Result<Self, FileSystemError> {
        Ok(DirEntry {
            uid: Uuid::new_v4(),
            name: EntryName::from_str(name)?,
            attributes: Attributes::from_preset(preset),
        })
    }
}
