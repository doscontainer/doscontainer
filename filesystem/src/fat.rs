use std::collections::BTreeMap;

use crate::{error::FileSystemError, ClusterIndex};

pub enum ClusterStatus {
    Allocated,
    Free,
    Bad,
    Reserved,
}

pub struct AllocationTable {
    clusters: BTreeMap<ClusterIndex, Cluster>,
    cluster_count: usize,
}

impl Default for AllocationTable {
    fn default() -> Self {
        AllocationTable {
            clusters: BTreeMap::new(),
            cluster_count: 0,
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
        if index  >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }

        match self.clusters.get(&index) {
            Some(ClusterStatus::Allocated) => return Err(FileSystemError::ClusterAlreadyAllocated),
            Some(ClusterStatus::Bad | ClusterStatus::Reserved) => {
                return Err(FileSystemError::ClusterNotUsable)
            }
            _ => {}
        }

        self.clusters.insert(index, ClusterStatus::Allocated);
        Ok(())
    }

    pub fn is_free(&self, index: ClusterIndex) -> bool {
        match self.clusters.get(&index) {
            Some(ClusterStatus::Free) => true,
            None => true,
            _ => false,
        }
    }
}
