#![allow(dead_code)]

use chrono::NaiveDateTime;
use error::FileSystemError;

mod allocationtable;
mod attributes;
mod bpb;
mod direntry;
mod error;
pub mod fat12;
mod names;
mod pool;
pub mod serializer;

// Cluster index into the FAT
pub type ClusterIndex = usize;

#[cfg(test)]
mod tests;

pub trait FileSystem {
    /// Create a new file
    fn mkfile(
        &mut self,
        path: &str,
        filesize: usize,
        creation_time: Option<NaiveDateTime>,
    ) -> Result<(), FileSystemError>;

    /// Create a new system file
    fn mksysfile(
        &mut self,
        path: &str,
        filesize: usize,
        creation_time: Option<NaiveDateTime>,
    ) -> Result<(), FileSystemError>;

    /// Create a directory
    fn mkdir(
        &mut self,
        path: &str,
        entries_count: usize,
        creation_time: Option<NaiveDateTime>,
    ) -> Result<(), FileSystemError>;
}
