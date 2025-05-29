use disk::error::DiskError;

#[derive(Debug)]
pub enum BuildError {
    DiskIoError(DiskError),
    FileSystemError,
}

impl From<DiskError> for BuildError {
    fn from(err: DiskError) -> Self {
        BuildError::DiskIoError(err)
    }
}
