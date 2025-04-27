#[derive(Debug)]
pub enum HwSpecError {
    ClockTooLow,
    ClockTooHigh,
    DuplicateAudioDevice,
    DuplicateVideoDevice,
    InvalidCpu,
    InvalidFloppyType,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidStorageClass,
    InvalidVideoDevice,
}
