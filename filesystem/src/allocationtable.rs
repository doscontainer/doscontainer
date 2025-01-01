use std::collections::BTreeMap;

use crate::{
    cluster::{Cluster, ClusterStatus},
    error::FileSystemError,
    ClusterIndex,
};

pub struct AllocationTable {
    clusters: BTreeMap<ClusterIndex, Cluster>,
    cluster_count: usize,
    cluster_size: usize,
}

impl AllocationTable {
    /// Creates a new allocation table.
    ///
    /// # Arguments
    /// - `cluster_count`: The total number of clusters in the filesystem.
    /// - `cluster_size`: The size of each cluster in bytes.
    pub fn new(cluster_count: usize, cluster_size: usize) -> Self {
        let clusters = (0..cluster_count)
            .map(|i| (i, Cluster::default()))
            .collect();

        Self {
            clusters,
            cluster_count,
            cluster_size,
        }
    }

    pub fn allocate_cluster(
        &mut self,
        index: ClusterIndex,
        value: usize,
    ) -> Result<(), FileSystemError> {
        // Guard clause: Ensure the index is within bounds.
        let cluster_count = self.cluster_count; // Copy this from self so that the error message can use it.
        if index >= self.cluster_count {
            return Err(FileSystemError::ClusterOutOfBounds {
                index,
                cluster_count,
            });
        }

        // Insert the cluster if it's not already in the map and then get a mutable reference.
        let cluster = self.clusters.entry(index).or_default();

        // Check if the cluster is free or reserved before allocating.
        if cluster.status() != ClusterStatus::Free && cluster.status() != ClusterStatus::Reserved {
            return Err(FileSystemError::ClusterNotFree { index });
        }

        // Perform the allocation.
        cluster.allocate(value);
        Ok(())
    }
}
