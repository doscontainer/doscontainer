use std::str::FromStr;

use chrono::{Local, NaiveDateTime};
use uuid::Uuid;

use crate::{
    attributes::{Attributes, AttributesPreset},
    error::FileSystemError,
    names::EntryName,
    ClusterIndex,
};

#[derive(Debug, PartialEq)]
pub struct DirEntry {
    uid: Uuid,
    entry_type: DirEntryType,
    parent: Option<Uuid>,
    attributes: Attributes,
    name: Option<EntryName>,
    creation_time: NaiveDateTime,
    start_cluster: Option<ClusterIndex>,
    cluster_map: Vec<ClusterIndex>,
    file_size: usize,
}

#[derive(Debug, PartialEq)]
pub enum DirEntryType {
    File,
    Directory,
    VolumeLabel,
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
            entry_type: DirEntryType::Directory,
            attributes: Attributes::from_preset(AttributesPreset::Directory),
            name: None,
            creation_time: Local::now().naive_local(),
            start_cluster: None,
            cluster_map: Vec::new(),
            file_size: 0,
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

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }
    pub fn file_size(&self) -> usize {
        self.file_size
    }

    pub fn start_cluster(&self) -> Option<usize> {
        self.start_cluster
    }
    pub fn name(&self) -> Option<&EntryName> {
        self.name.as_ref()
    }

    pub fn creation_time(&self) -> NaiveDateTime {
        self.creation_time
    }

    pub fn set_creation_time(&mut self, creation_time: NaiveDateTime) {
        self.creation_time = creation_time;
    }
    pub fn set_start_cluster(&mut self, start_cluster: usize) {
        println!("Setting start cluster to: {}", start_cluster);
        self.start_cluster = Some(start_cluster);
    }

    pub fn set_filesize(&mut self, filesize: usize) {
        self.file_size = filesize;
    }

    pub fn set_cluster_map(&mut self, cluster_map: &[ClusterIndex]) {
        self.cluster_map = cluster_map.to_vec();
    }

    pub fn cluster_map(&self) -> &[ClusterIndex] {
        &self.cluster_map
    }

    /// Is the entry a file?
    pub fn is_file(&self) -> bool {
        matches!(self.entry_type, DirEntryType::File)
    }

    /// Is the entry a directory?
    pub fn is_directory(&self) -> bool {
        matches!(self.entry_type, DirEntryType::Directory)

    }

    fn new_from_preset(name: &str, preset: AttributesPreset) -> Result<Self, FileSystemError> {
        let entry_type = match preset {
            AttributesPreset::Directory => DirEntryType::Directory,
            AttributesPreset::RegularFile | AttributesPreset::SystemFile => DirEntryType::File,
            AttributesPreset::VolumeLabel => DirEntryType::VolumeLabel,
        };
        Ok(DirEntry {
            uid: Uuid::new_v4(),
            entry_type,
            parent: None,
            name: Some(EntryName::from_str(name)?),
            attributes: Attributes::from_preset(preset),
            creation_time: Local::now().naive_local(),
            start_cluster: None,
            cluster_map: Vec::new(),
            file_size: 0,
        })
    }
}
