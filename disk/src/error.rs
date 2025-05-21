#[derive(Debug,PartialEq)]
pub enum DiskError {
    BufferTooSmall,
    FileMetadataFailed,
    FileOpenFailed,
    FlushFailed,
    InvalidFileSize,
    InvalidSectorSize,
    OutOfBounds,
    ReadFailed,
    SeekFailed,
    WriteFailed,
}