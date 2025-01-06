#[derive(Debug)]
pub enum CoreError {
    ChecksumError,
    CreateFileError,
    CreateDirError,
    DownloadError,
    FileReadError,
    PermissionError,
    ZipFileOpenError,
    ZipFileWriteError,
}