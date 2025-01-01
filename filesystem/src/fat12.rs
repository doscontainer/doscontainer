use crate::{allocationtable::AllocationTable, pool::Pool, FileSystem};
use operatingsystem::OperatingSystem;

pub struct Fat12 {
    allocation_table: AllocationTable,
    pool: Pool,
    os: OperatingSystem,
    sector_count: usize,
}

impl FileSystem for Fat12 {
    fn mkfile<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
        size: usize,
        filetype: crate::FileType,
    ) -> Result<Vec<crate::ClusterIndex>, crate::error::FileSystemError> {
        todo!()
    }

    fn mkdir<P: AsRef<std::path::Path>>(
        &mut self,
        path: P,
    ) -> Result<Vec<crate::ClusterIndex>, crate::error::FileSystemError> {
        todo!()
    }
}
