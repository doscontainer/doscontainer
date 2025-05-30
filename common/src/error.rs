use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommonError {
    #[error("Clock rate is too high")]
    ClockTooHigh,

    #[error("Clock rate is too low")]
    ClockTooLow,

    #[error("Invalid audio device: {0}")]
    InvalidAudioDevice(String),

    #[error("Invalid CPU specified")]
    InvalidCpu,

    #[error("Invalid floppy type")]
    InvalidFloppyType,

    #[error("Invalid video device.")]
    InvalidVideoDevice,
}