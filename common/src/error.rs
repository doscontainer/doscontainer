use std::fmt;

#[derive(Debug)]
pub enum CommonError {
    ClockTooHigh,
    ClockTooLow,
    InvalidAudioDevice(String),
    InvalidCpu,
    InvalidFloppyType,
    InvalidVideoDevice,
}


impl fmt::Display for CommonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CommonError::ClockTooHigh => write!(f, "Clock rate is too high"),
            CommonError::ClockTooLow => write!(f, "Clock rate is too low"),
            CommonError::InvalidAudioDevice(dev) => write!(f, "Invalid audio device: {}", dev),
            CommonError::InvalidCpu => write!(f, "Invalid CPU specified"),
            CommonError::InvalidFloppyType => write!(f, "Invalid floppy type"),
            CommonError::InvalidVideoDevice => write!(f, "Invalid video device."),
        }
    }
}
