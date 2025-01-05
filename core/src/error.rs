#[derive(Debug)]
pub enum CoreError {
    ChecksumError,
    DownloadError,
    FileReadError,
}