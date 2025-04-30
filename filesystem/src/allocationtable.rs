use crate::{
    cluster::{Cluster, ClusterStatus},
    error::FileSystemError,
    ClusterIndex,
};
use std::collections::BTreeMap;

pub struct AllocationTable {
    clusters: BTreeMap<ClusterIndex, Cluster>,
    cluster_size: usize,
}

impl AllocationTable {
    /// Simple getter for the table's cluster size (in bytes)
    pub fn cluster_size(&self) -> usize {
        self.cluster_size
    }

    /// Retrieves the status of a cluster by its index.
    ///
    /// This method checks the allocation table for a cluster at the given `index`. If the cluster
    /// exists, its status is returned. Otherwise, an error is returned indicating that the index
    /// is out of bounds.
    ///
    /// # Arguments
    /// - `index`: The index of the cluster whose status is to be retrieved.
    ///
    /// # Errors
    /// Returns `FileSystemError::ClusterOutOfBounds` if the specified `index` is outside the bounds
    /// of the allocation table.
    ///
    /// # Returns
    /// - `Ok(ClusterStatus)` — the status of the cluster at the specified index
    /// - `Err(FileSystemError::ClusterOutOfBounds)` — if the index is invalid
    pub fn cluster_status(&self, index: usize) -> Result<ClusterStatus, FileSystemError> {
        if let Some(cluster) = self.clusters.get(&index) {
            Ok(cluster.status())
        } else {
            Err(FileSystemError::ClusterOutOfBounds {
                index,
                cluster_count: self.clusters.len(),
            })
        }
    }

    /// Finds the first free cluster in the allocation table, marks it as reserved,
    /// and returns its index.
    ///
    /// This method iterates over the clusters in order and selects the first one
    /// with `ClusterStatus::Free`. Once found, the cluster is marked as reserved
    /// (via its `reserve()` method) and its index is returned.
    ///
    /// # Errors
    ///
    /// Returns `FileSystemError::AllocationTableFull` if no free clusters are available.
    ///
    /// # Returns
    ///
    /// - `Ok(usize)` — the index of the reserved cluster
    /// - `Err(FileSystemError::AllocationTableFull)` — if the allocation table is full
    pub fn reserve_cluster(&mut self) -> Result<usize, FileSystemError> {
        for (&index, cluster) in self.clusters.iter_mut() {
            if cluster.status() == ClusterStatus::Free {
                cluster.reserve();
                return Ok(index);
            }
        }
        Err(FileSystemError::AllocationTableFull)
    }

    /// Creates a new allocation table.
    ///
    /// # Arguments
    /// - `cluster_count`: The total number of clusters in the filesystem.
    /// - `cluster_size`: The size of each cluster in bytes.
    pub fn new(cluster_count: usize, cluster_size: usize) -> Result<Self, FileSystemError> {
        if cluster_count == 0 {
            return Err(FileSystemError::NoAllocatedClusters);
        }
        let clusters = (0..cluster_count)
            .map(|i| (i, Cluster::default()))
            .collect();
        Ok(Self {
            clusters,
            cluster_size,
        })
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
        if index >= self.clusters.len() {
            return Err(FileSystemError::ClusterOutOfBounds {
                index,
                cluster_count: self.clusters.len(),
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
