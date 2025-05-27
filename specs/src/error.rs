use std::fmt;

#[derive(Debug)]
pub enum SpecError {
    ChecksumVerificationFailed,
    ClockTooLow,
    ClockTooHigh,
    ConfigBuild(config::ConfigError),
    Deserialize(config::ConfigError),
    DownloadError,
    DuplicateAudioDevice,
    DuplicateVideoDevice,
    FileOpenError,
    FtpAuthenticationError,
    FtpConnectionError,
    FtpTransferTypeError,
    HttpRequestError,
    InvalidAudioDevice(String),
    InvalidCpu,
    InvalidFileSystemType,
    InvalidFloppyType,
    InvalidUrl,
    MissingUrl,
    NoFloppyDrive,
    TempDirError,
    TooManyCylinders,
    TooManyHeads,
    TooManySectors,
    TooMuchRamSpecified,
    UnsupportedUrlScheme,
    InvalidRamString,
    InvalidStorageClass,
    InvalidVideoDevice,
    TomlLoadError(String),
    ValueMayNotBeZero,
    ZipFileCorrupt,
    ZipFileNotSet,
}

impl fmt::Display for SpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SpecError::ChecksumVerificationFailed => write!(f, "Checksum verification failed."),
            SpecError::ClockTooLow => write!(f, "Specified clock speed is too low"),
            SpecError::ClockTooHigh => write!(f, "Specified clock speed is too high"),
            SpecError::ConfigBuild(err) => write!(f, "Failed parsing configuration: {err}"),
            SpecError::Deserialize(err) => write!(f, "Failed deserializing configuration: {err}"),
            SpecError::DownloadError => write!(f, "Download error."),
            SpecError::DuplicateAudioDevice => write!(f, "Duplicate audio device specified"),
            SpecError::DuplicateVideoDevice => write!(f, "Duplicate video device specified"),
            SpecError::FileOpenError => write!(f, "Error opening file."),
            SpecError::FtpAuthenticationError => write!(f, "FTP authentication error."),
            SpecError::FtpConnectionError => write!(f, "FTP connection error."),
            SpecError::FtpTransferTypeError => write!(f, "FTP transfer type error."),
            SpecError::HttpRequestError => write!(f, "HTTP request error."),
            SpecError::InvalidCpu => write!(f, "Invalid CPU model specified"),
            SpecError::InvalidFloppyType => write!(f, "Invalid floppy drive type specified"),
            SpecError::InvalidUrl => write!(f, "Invalid URL"),
            SpecError::InvalidFileSystemType => write!(f, "Invalid file system type."),
            SpecError::MissingUrl => write!(f, "Missing URL"),
            SpecError::NoFloppyDrive => write!(f, "No floppy drive defined on system"),
            SpecError::TempDirError => write!(f, "Error handling temporary directory."),
            SpecError::TooManyCylinders => {
                write!(f, "Too many cylinders specified for storage device")
            }
            SpecError::TooManyHeads => write!(f, "Too many heads specified for storage device"),
            SpecError::TooManySectors => write!(f, "Too many sectors per track specified"),
            SpecError::TooMuchRamSpecified => {
                write!(f, "Too much RAM specified (maximum is 4 GiB)")
            }
            SpecError::UnsupportedUrlScheme => write!(f, "Unsupported URL scheme"),
            SpecError::InvalidAudioDevice(msg) => {
                write!(f, "Invalid audio device specified: {}.", msg)
            }
            SpecError::InvalidRamString => write!(f, "Invalid RAM string format"),
            SpecError::InvalidStorageClass => write!(f, "Invalid storage class specified"),
            SpecError::InvalidVideoDevice => write!(f, "Invalid video device specified"),
            SpecError::TomlLoadError(msg) => write!(f, "TOML load error: {}", msg),
            SpecError::ValueMayNotBeZero => write!(f, "Value may not be zero"),
            SpecError::ZipFileCorrupt => write!(f, "ZIP file corruption error."),
            SpecError::ZipFileNotSet => write!(f, "ZIP file not set."),
        }
    }
}
