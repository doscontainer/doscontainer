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
    cluster_size: usize,
    cluster_count: usize,
    fat_type: FatType,
}

impl Default for AllocationTable {
    fn default() -> Self {
        AllocationTable {
            clusters: BTreeMap::new(),
            cluster_size: 512,
            cluster_count: 340,
            fat_type: FatType::Fat12,
        }
    }
}

impl AllocationTable {
    pub fn clusters(&self) -> &BTreeMap<ClusterIndex, ClusterValue> {
        &self.clusters
    }

    pub fn set_cluster_count(&mut self, cluster_count: usize) -> Result<(), FileSystemError> {
        if cluster_count < self.cluster_count {
            return Err(FileSystemError::WontShrinkAllocationTable);
        }
        let max_cluster_count = match self.fat_type {
            FatType::Fat12 => 4096,
        };

        if cluster_count > max_cluster_count {
            return Err(FileSystemError::FatSizeTooLarge);
        }

        self.cluster_count = cluster_count;
        Ok(())
    }

    pub fn allocate_entry(&mut self, size: usize) -> Result<Vec<ClusterIndex>, FileSystemError> {
        // Always allocate at least one cluster
        let clusters_needed = std::cmp::max(1, size.div_ceil(self.cluster_size));

        let mut free_clusters = Vec::with_capacity(clusters_needed);
        for index in 0..self.cluster_count {
            if self.is_free(index)? {
                free_clusters.push(index);
                if free_clusters.len() == clusters_needed {
                    break;
                }
            }
        }

        if free_clusters.len() < clusters_needed {
            return Err(FileSystemError::NotEnoughFreeClusters);
        }

        for i in 0..clusters_needed {
            let current = free_clusters[i];
            let next = if i + 1 < clusters_needed {
                Some(free_clusters[i + 1])
            } else {
                None
            };
            self.allocate(current, next)?;
        }

        Ok(free_clusters)
    }

    pub fn allocate(
        &mut self,
        index: ClusterIndex,
        next: Option<ClusterIndex>,
    ) -> Result<(), FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }

        match self.clusters.get(&index) {
            Some(ClusterValue::Next(_)) | Some(ClusterValue::EndOfChain) => {
                return Err(FileSystemError::ClusterAlreadyAllocated)
            }
            Some(ClusterValue::Bad) | Some(ClusterValue::Reserved) => {
                return Err(FileSystemError::ClusterNotUsable)
            }
            Some(ClusterValue::Free) | None => {
                let value = match next {
                    Some(n) => ClusterValue::Next(n),
                    None => ClusterValue::EndOfChain,
                };
                self.clusters.insert(index, value);
            }
        }

        Ok(())
    }

    pub fn reserve(&mut self, index: ClusterIndex) -> Result<(), FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }

        if !self.is_free(index)? {
            return Err(FileSystemError::ClusterNotUsable);
        } else {
            self.clusters.insert(index, ClusterValue::Reserved);
        }
        Ok(())
    }

    pub fn mark_end_of_chain(&mut self, index: ClusterIndex) -> Result<(), FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }

        match self.clusters.get(&index) {
            Some(ClusterValue::Free) | None => {
                self.clusters.insert(index, ClusterValue::EndOfChain);
                Ok(())
            }
            _ => Err(FileSystemError::ClusterNotUsable),
        }
    }

    pub fn is_free(&self, index: ClusterIndex) -> Result<bool, FileSystemError> {
        if index >= self.cluster_count {
            return Err(FileSystemError::InvalidClusterIndex);
        }
        match self.clusters.get(&index) {
            Some(ClusterValue::Free) | None => Ok(true),
            _ => Ok(false),
        }
    }
}
