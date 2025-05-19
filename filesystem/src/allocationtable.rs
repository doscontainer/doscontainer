use std::collections::BTreeMap;

use crate::{error::FileSystemError, ClusterIndex};

#[derive(Debug)]
pub enum ClusterValue {
    Next(ClusterIndex),
    EndOfChain,
    Free,
    Reserved,
    Bad,
}

#[derive(Debug)]
pub enum FatType {
    Fat12,
}

#[derive(Debug)]
pub struct AllocationTable {
    clusters: BTreeMap<ClusterIndex, ClusterValue>,
    cluster_count: usize,
    fat_type: FatType,
}

impl Default for AllocationTable {
    fn default() -> Self {
        AllocationTable {
            clusters: BTreeMap::new(),
            cluster_count: 0,
            fat_type: FatType::Fat12,
        }
    }
}

impl AllocationTable {
    pub fn set_cluster_count(&mut self, cluster_count: usize) -> Result<(), FileSystemError> {
        if cluster_count < self.cluster_count {
            return Err(FileSystemError::WontShrinkAllocationTable);
        }
        self.cluster_count = cluster_count;
        Ok(())
    }

    pub fn allocate(&mut self, index: ClusterIndex) -> Result<(), FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }

        match self.clusters.get(&index) {
            Some(ClusterValue::Next(_)) | Some(ClusterValue::EndOfChain) => {
                return Err(FileSystemError::ClusterAlreadyAllocated)
            }
            Some(ClusterValue::Bad) => return Err(FileSystemError::ClusterNotUsable),
            Some(ClusterValue::Reserved) => return Err(FileSystemError::ClusterNotUsable),
            Some(ClusterValue::Free) | None => {
                // Ok to allocate
            }
        }

        self.clusters.insert(index, ClusterValue::EndOfChain); // Start a new chain
        Ok(())
    }

    pub fn is_free(&self, index: ClusterIndex) -> Result<bool, FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }
        match self.clusters.get(&index) {
            Some(ClusterValue::Free) | None => Ok(true),
            Some(ClusterValue::Bad) | Some(ClusterValue::Reserved) | _ => Ok(false),
        }
    }

    pub fn is_allocated(&self, index: ClusterIndex) -> Result<bool, FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }
        match self.clusters.get(&index) {
            Some(ClusterValue::Next(_))
            | Some(ClusterValue::EndOfChain)
            | Some(ClusterValue::Reserved)
            | Some(ClusterValue::Bad) => Ok(true),
            _ => Ok(false),
        }
    }
}
