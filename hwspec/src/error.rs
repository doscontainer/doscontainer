#[derive(Debug)]
pub enum HwSpecError {
    ClockTooLow,
    ClockTooHigh,
    DuplicateAudioDevice,
    DuplicateVideoDevice,
    InvalidCpu,
    InvalidFloppyType,
    TooManyCylinders,
    TooManyHeads,
    TooManySectors,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidStorageClass,
    InvalidVideoDevice,
    ValueMayNotBeZero,
}
