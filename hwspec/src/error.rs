#[derive(Debug)]
pub enum HwSpecError {
    DuplicateAudioDevice,
    DuplicateVideoDevice,
    InvalidCpu,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidVideoDevice,
}
