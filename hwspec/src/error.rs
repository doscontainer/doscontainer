#[derive(Debug)]
pub enum HwSpecError {
    DuplicateAudioDevice,
    InvalidCpu,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidVideoDevice,
}
