use std::collections::BTreeMap;
use crate::{cluster::{Cluster, ClusterStatus}, error::FileSystemError, ClusterIndex};

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

    /// Allocates a new cluster at the specified index with the given value.
    /// The cluster must be free or reserved to be allocated.
    ///
    /// # Arguments
    /// - `index`: The index of the cluster to allocate.
    /// - `value`: The value to associate with the newly allocated cluster.
    ///
    /// # Returns
    /// A `Result` indicating whether the allocation was successful or if an error occurred.
    pub fn allocate_cluster(
        &mut self,
        index: ClusterIndex,
        value: usize,
    ) -> Result<(), FileSystemError> {
        // Ensure the index is within bounds.
        if index >= self.cluster_count {
            return Err(FileSystemError::ClusterOutOfBounds {
                index,
                cluster_count: self.cluster_count,
            });
        }

        // Get a mutable reference to the cluster entry in the map.
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