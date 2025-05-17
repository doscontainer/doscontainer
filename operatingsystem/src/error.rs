/// This module contains definitions related to operating system errors.
use std::fmt;

#[derive(Debug)]
/// Represents various error conditions that can occur in OS-related operations.
pub enum OsError {
    /// Indicates an attempt was made to use a BPB (BIOS Parameter Block) on an
    /// incompatible operating system or file system where it is not applicable.
    BpbNotApplicable,
    /// Error when the OS product string is invalid. This can occur if the
    /// product identifier doesn't match known valid formats.
    InvalidOsProduct(String),
    /// Error when the OS vendor information is invalid or cannot be parsed.
    InvalidOsVendor(String),
    /// Represents an error condition when the OS version format is invalid
    /// or cannot be parsed into a valid version number.
    InvalidOsVersionFormat(String),
    /// Indicates there was an issue with an URL (e.g., download failed due to
    /// invalid or unreachable URL).
    InvalidUrl,
    /// Error when attempting to treat a non-floppy disk as a floppy drive.
    NotAFloppy,
    /// Indicates that the type of disk media is not supported by this OS or
    /// file system implementation.
    UnsupportedDiskType,
    /// Represents an error condition where the operating system itself is not
    /// supported by this implementation (e.g., trying to run on a unsupported
    /// OS version or platform).
    UnsupportedOs,
}

/// Implements the Display trait for OsError enum, providing user-friendly
/// error messages that describe the specific issue encountered.
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
