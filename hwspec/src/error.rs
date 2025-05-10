use std::fmt;

#[derive(Debug)]
pub enum HwSpecError {
    ClockTooLow,
    ClockTooHigh,
    ConfigBuild(config::ConfigError),
    Deserialize(config::ConfigError),
    DuplicateAudioDevice,
    DuplicateVideoDevice,
    InvalidAudioDevice(String),
    InvalidCpu,
    InvalidFloppyType,
    TooManyCylinders,
    TooManyHeads,
    TooManySectors,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidStorageClass,
    InvalidVideoDevice,
    TomlLoadError(String),
    ValueMayNotBeZero,
}

impl fmt::Display for HwSpecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HwSpecError::ClockTooLow => write!(f, "Specified clock speed is too low"),
            HwSpecError::ClockTooHigh => write!(f, "Specified clock speed is too high"),
            HwSpecError::ConfigBuild(err) => write!(f, "Failed parsing configuration: {err}"),
            HwSpecError::Deserialize(err) => write!(f, "Failed deserializing configuration: {err}"),
            HwSpecError::DuplicateAudioDevice => write!(f, "Duplicate audio device specified"),
            HwSpecError::DuplicateVideoDevice => write!(f, "Duplicate video device specified"),
            HwSpecError::InvalidCpu => write!(f, "Invalid CPU model specified"),
            HwSpecError::InvalidFloppyType => write!(f, "Invalid floppy drive type specified"),
            HwSpecError::TooManyCylinders => {
                write!(f, "Too many cylinders specified for storage device")
            }
            HwSpecError::TooManyHeads => write!(f, "Too many heads specified for storage device"),
            HwSpecError::TooManySectors => write!(f, "Too many sectors per track specified"),
            HwSpecError::TooMuchRamSpecified => {
                write!(f, "Too much RAM specified (maximum is 4 GiB)")
            }
            HwSpecError::InvalidAudioDevice(msg) => {
                write!(f, "Invalid audio device specified: {}.", msg)
            }
            HwSpecError::InvalidRamString => write!(f, "Invalid RAM string format"),
            HwSpecError::InvalidStorageClass => write!(f, "Invalid storage class specified"),
            HwSpecError::InvalidVideoDevice => write!(f, "Invalid video device specified"),
            HwSpecError::TomlLoadError(msg) => write!(f, "TOML load error: {}", msg),
            HwSpecError::ValueMayNotBeZero => write!(f, "Value may not be zero"),
        }
    }
}
