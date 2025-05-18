use std::str::FromStr;

use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

use crate::{
    attributes::{Attributes, AttributesPreset},
    error::FileSystemError,
    names::EntryName, ClusterIndex,
};

#[derive(Debug, PartialEq)]
pub struct DirEntry {
    uid: Uuid,
    parent: Option<Uuid>,
    attributes: Attributes,
    name: Option<EntryName>,
    creation_time: NaiveDateTime,
    start_cluster: ClusterIndex,
    file_size: usize,
    can_be_parent: bool,
}

impl DirEntry {
    /// Create a regular file
    pub fn new_file(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::RegularFile)
    }

    /// This one's special: there can be only one root directory in a Pool.
    /// Create it through this constructor.
    pub fn new_rootdir() -> Self {
        Self {
            uid: Uuid::new_v4(),
            parent: None,
            attributes: Attributes::from_preset(AttributesPreset::Directory),
            name: None,
            creation_time: Local::now().naive_local(),
            start_cluster: 0,
            file_size: 0,
            can_be_parent: true,
        }
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

    pub fn uuid(&self) -> &Uuid {
        &self.uid
    }

    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    pub fn set_parent(&mut self, parent: &DirEntry) {
        self.parent = Some(*parent.uuid());
    }

    /// Check whether the current entry is the root node
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    /// Check if I can accept child entries
    pub fn can_be_parent(&self) -> bool {
        self.can_be_parent
    }

    fn new_from_preset(name: &str, preset: AttributesPreset) -> Result<Self, FileSystemError> {
        let can_be_parent = matches!(preset, AttributesPreset::Directory);
        Ok(DirEntry {
            uid: Uuid::new_v4(),
            parent: None,
            name: Some(EntryName::from_str(name)?),
            attributes: Attributes::from_preset(preset),
            creation_time: Local::now().naive_local(),
            start_cluster: 0,
            file_size: 0,
            can_be_parent,
        })
    }
}
