use std::fmt;

use crate::ClusterIndex;

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct Cluster {
    index: ClusterIndex,
    status: ClusterStatus,
}

#[derive(Copy, Clone, Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum ClusterStatus {
    Allocated,
    Free,
    Bad,
    Reserved,
}

impl fmt::Display for ClusterStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Free => write!(f, "Free"),
            Self::Reserved => write!(f, "Reserved"),
            Self::Bad => write!(f, "Bad"),
            Self::Allocated => write!(f, "Allocated"),
        }
    }
}

impl Default for Cluster {
    fn default() -> Self {
        Self::new()
    }
}

impl Cluster {
    pub fn new() -> Self {
        // Every cluster starts as Free, having value 0.
        Self {
            index: 0,
            status: ClusterStatus::Free,
        }
    }

    pub fn bad(&mut self) {
        self.index = 0;
        self.status = ClusterStatus::Bad;
    }

    pub fn allocate(&mut self, index: ClusterIndex) {
        self.index = index;
        self.status = ClusterStatus::Allocated;
    }

    pub fn free(&mut self) {
        self.index = 0;
        self.status = ClusterStatus::Free;
    }

    pub fn reserve(&mut self) {
        self.status = ClusterStatus::Reserved;
    }

    pub fn value(&self) -> usize {
        self.index
    }

    pub fn status(&self) -> ClusterStatus {
        self.status
    }
}
