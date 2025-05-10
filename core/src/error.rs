#[derive(Debug)]
pub enum CoreError {
    Checksum,
    CreateFile,
    CreateDir,
    DiskType,
    Download,
    FileRead,
    OsInstall,
    Permission,
    ZipFileOpen,
    ZipFileWrite,
}
