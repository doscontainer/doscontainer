use std::fmt;

#[derive(Debug)]
pub enum OsError {
    BpbNotApplicable,
    InvalidOsProduct(String),
    InvalidOsVendor(String),
    InvalidOsVersionFormat(String),
    InvalidUrl,
    NotAFloppy,
    UnsupportedDiskType,
    UnsupportedOs,
}

impl fmt::Display for OsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use OsError::*;
        match self {
            BpbNotApplicable => write!(f, "BPB not applicable for this OS."),
            InvalidOsProduct(err) => write!(f, "Invalid OS product: {}", err),
            InvalidOsVendor(err) => write!(f, "Invalid OS vendor : {}", err),
            InvalidOsVersionFormat(err) => write!(f, "Invalid OS version format: {}", err),
            InvalidUrl => write!(f, "Invalid download URL"),
            NotAFloppy => write!(f, "Not a floppy."),
            UnsupportedDiskType => write!(f, "Unsupported disk type."),
            UnsupportedOs => write!(f, "Unsupported Operating System."),
        }
    }
}
