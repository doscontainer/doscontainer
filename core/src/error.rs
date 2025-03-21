#[derive(Debug)]
pub enum CoreError {
    ChecksumError,
    CreateFileError,
    CreateDirError,
    DiskTypeError,
    DownloadError,
    FileReadError,
    OsInstallError,
    PermissionError,
    ZipFileOpenError,
    ZipFileWriteError,
}
