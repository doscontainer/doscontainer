#[derive(Debug, PartialEq)]
pub enum DiskError {
    BufferTooSmall,
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
