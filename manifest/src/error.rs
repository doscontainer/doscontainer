#[derive(Debug)]
pub enum ManifestError {
    DownloadError,
    FtpAuthenticationError,
    FtpConnectionError,
    FtpTransferTypeError,
    HttpRequestError,
    InvalidDiskCategory,
    InvalidDiskType,
    InvalidLayerType,
    InvalidUrl,
    MissingUrl,
    TempDirError,
    UnsupportedUrlScheme,
}
