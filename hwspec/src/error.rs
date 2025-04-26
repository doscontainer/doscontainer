#[derive(Debug)]
pub enum HwSpecError {
    InvalidCpu,
    TooMuchRamSpecified,
    InvalidRamString,
    InvalidVideoDevice,
}
