#[derive(Debug, PartialEq)]
pub enum DiskError {
    BufferTooSmall,
    FileAlreadyExists,
    FileMetadataFailed,
    FileOpenFailed,
    FlushFailed,
    InvalidArgument,
    InvalidFileSize,
    InvalidSectorSize,
    IoError,
    OutOfBounds,
    ReadFailed,
    SeekFailed,
    WriteFailed,
}
