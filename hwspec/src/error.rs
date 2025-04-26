#[derive(Debug)]
pub enum HwSpecError {
    ClockTooLow,
    ClockTooHigh,
    DuplicateAudioDevice,
    DuplicateVideoDevice,
    InvalidCpu,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidVideoDevice,
}
