#[derive(Debug)]
pub enum CoreError {
    ChecksumError,
    CreateFileError,
    CreateDirError,
    DiskTypeError,
    DownloadError,
    FileReadError,
    PermissionError,
    ZipFileOpenError,
    ZipFileWriteError,
}