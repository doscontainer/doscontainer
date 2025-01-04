use std::fmt;

#[derive(Debug, PartialEq)]
pub enum DownloadError {
    /// Filename part of URL is empty
    FileNameIsEmpty,
    /// Failed FTP authentication
    FtpAuthenticationError,
    /// FTP connection error
    FtpConnectionError,
    /// FTP disconnection error
    FtpDisconnectError,
    /// FTP stream read error
    FtpStreamReadError,
    /// Failed to set FTP transfer type
    FtpTransferTypeError,
    /// Invalid URL
    InvalidUrl,
    /// Failed to create local file
    LocalFileCreationError,
    /// Failed to write bytes to the local file
    LocalFileWriteError,
    /// Unsupported Schema
    UnsupportedScheme,
    /// Path-part of URL is empty
    PathIsEmpty,
    /// Creating the tempdir for zipfiles failed
    ZipDirCreateFailed,
    
}

impl std::fmt::Display for DownloadError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DownloadError::FtpAuthenticationError => { write!(f, "Failed authentication to FTP server, or server does not accept anonymous logins.")},
            DownloadError::FileNameIsEmpty => { write!(f, "Filename is empty.")},
            DownloadError::FtpConnectionError => { write!(f, "Unable to connect to FTP server.")},
            DownloadError::FtpDisconnectError => { write!(f, "Error occured when disconnecting from FTP server.")},
            DownloadError::FtpStreamReadError => { write!(f, "Error while reading the FTP data stream.")},
            DownloadError::FtpTransferTypeError => { write!(f, "Unable to set the FTP transfer type.")},
            DownloadError::InvalidUrl => { write!(f, "Invalid URL given.")},
            DownloadError::LocalFileCreationError => { write!(f, "Failed to create local file for download.")},
            DownloadError::LocalFileWriteError => { write!(f, "Failed to write bytes to the destination file.")}
            DownloadError::UnsupportedScheme => { write!(f, "Unsupported URI scheme.")},
            DownloadError::PathIsEmpty => { write!(f, "Path part of URL is empty.")}
            DownloadError::ZipDirCreateFailed => { write!(f, "Failed to create temporary directory for ZIP files.")}
        }
    }
}

impl std::error::Error for DownloadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
