use std::fmt;

#[derive(Debug)]
pub enum ManifestError {
    ChecksumVerificationFailed,
    ConfigBuild(config::ConfigError),
    Deserialize(config::ConfigError),
    DownloadError,
    FileOpenError,
    FtpAuthenticationError,
    FtpConnectionError,
    FtpTransferTypeError,
    HttpRequestError,
    InvalidDiskCategory,
    InvalidDiskType,
    InvalidFileSystemType,
    InvalidLayerType,
    InvalidOsProduct(String),
    InvalidOsVendor(String),
    InvalidOsVersionFormat(String),
    InvalidUrl,
    MissingUrl,
    MissingVendor,
    MissingVersion,
    StagingPathNotSet,
    TempDirError,
    UnsupportedUrlScheme,
    ZipFileCorrupt,
    ZipFileNotSet,
}

impl fmt::Display for ManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use ManifestError::*;
        match self {
            ChecksumVerificationFailed => write!(f, "Checksum verification failed."),
            ConfigBuild(err) => write!(f, "failed to build config: {}", err),
            Deserialize(err) => write!(f, "failed to deserialize config: {}", err),
            DownloadError => write!(f, "error downloading file"),
            FileOpenError => write!(f, "failed to open file"),
            FtpAuthenticationError => write!(f, "FTP authentication failed"),
            FtpConnectionError => write!(f, "could not connect to FTP server"),
            FtpTransferTypeError => write!(f, "failed to set FTP transfer type"),
            HttpRequestError => write!(f, "HTTP request failed"),
            InvalidDiskCategory => write!(f, "invalid disk category"),
            InvalidDiskType => write!(f, "invalid disk type"),
            InvalidFileSystemType => write!(f, "invalid filesystem type"),
            InvalidLayerType => write!(f, "invalid layer type"),
            InvalidOsProduct(err) => write!(f, "Invalid OS product: {}", err),
            InvalidOsVendor(err) => write!(f, "Invalid OS vendor : {}", err),
            InvalidOsVersionFormat(err) => write!(f, "Invalid OS version format: {}", err),
            InvalidUrl => write!(f, "invalid URL"),
            MissingUrl => write!(f, "missing URL"),
            MissingVendor => write!(f, "Missing OS vendor"),
            MissingVersion => write!(f, "Missing OS version."),
            StagingPathNotSet => write!(f, "staging path was not set"),
            TempDirError => write!(f, "could not create temporary directory"),
            UnsupportedUrlScheme => write!(f, "unsupported URL scheme"),
            ZipFileCorrupt => write!(f, "ZIP file is corrupt"),
            ZipFileNotSet => write!(f, "ZIP file path not set"),
        }
    }
}
