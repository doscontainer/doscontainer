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
    // --- Constructors ---

    pub fn new_file(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::RegularFile)
    }

    pub fn new_sysfile(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::SystemFile)
    }

    pub fn new_directory(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::Directory)
    }

    pub fn new_volume_label(name: &str) -> Result<Self, FileSystemError> {
        Self::new_from_preset(name, AttributesPreset::VolumeLabel)
    }

    /// Root directory constructor — only one should exist in a pool
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

    fn new_from_preset(name: &str, preset: AttributesPreset) -> Result<Self, FileSystemError> {
        let entry_type = match preset {
            AttributesPreset::Directory => DirEntryType::Directory,
            AttributesPreset::RegularFile | AttributesPreset::SystemFile => DirEntryType::File,
            AttributesPreset::VolumeLabel => DirEntryType::VolumeLabel,
        };

        Ok(Self {
            uid: Uuid::new_v4(),
            entry_type,
            parent: None,
            attributes: Attributes::from_preset(preset),
            name: Some(EntryName::from_str(name)?),
            creation_time: Local::now().naive_local(),
            start_cluster: None,
            cluster_map: Vec::new(),
            file_size: 0,
        })
    }

    // --- Getters ---

    pub fn uuid(&self) -> &Uuid {
        &self.uid
    }

    pub fn parent(&self) -> Option<&Uuid> {
        self.parent.as_ref()
    }

    pub fn name(&self) -> Option<&EntryName> {
        self.name.as_ref()
    }

    pub fn attributes(&self) -> &Attributes {
        &self.attributes
    }

    pub fn creation_time(&self) -> NaiveDateTime {
        self.creation_time
    }

    pub fn start_cluster(&self) -> Option<usize> {
        self.start_cluster
    }

    pub fn file_size(&self) -> usize {
        self.file_size
    }

    pub fn cluster_map(&self) -> &[ClusterIndex] {
        &self.cluster_map
    }

    pub fn is_file(&self) -> bool {
        matches!(self.entry_type, DirEntryType::File)
    }

    pub fn is_directory(&self) -> bool {
        matches!(self.entry_type, DirEntryType::Directory)
    }

    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }

    // --- Setters ---

    pub fn set_parent(&mut self, parent: &DirEntry) {
        self.parent = Some(*parent.uuid());
    }

    pub fn set_creation_time(&mut self, creation_time: NaiveDateTime) {
        self.creation_time = creation_time;
    }

    pub fn set_start_cluster(&mut self, start_cluster: usize) {
        self.start_cluster = Some(start_cluster);
    }

    pub fn set_filesize(&mut self, filesize: usize) {
        self.file_size = filesize;
    }

    pub fn set_cluster_map(&mut self, cluster_map: &[ClusterIndex]) {
        self.cluster_map = cluster_map.to_vec();
    }
}
