use crate::OsError;
use std::fmt;
use std::str::FromStr;

/// Operating System product name in a type-safe way.
#[derive(Debug)]
pub enum OsProduct {
    DrDos,
    FreeDOS,
    MsDos,
    PcDos,
}

impl fmt::Display for OsProduct {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OsProduct::DrDos => write!(f, "DR-DOS"),
            OsProduct::FreeDOS => write!(f, "FreeDOS"),
            OsProduct::MsDos => write!(f, "MS-DOS"),
            OsProduct::PcDos => write!(f, "PC-DOS"),
        }
    }
}

impl FromStr for OsProduct {
    type Err = OsError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim().to_lowercase().as_str() {
            "msdos" | "ms-dos" | "ms dos" => Ok(OsProduct::MsDos),
            "pcdos" | "pc-dos" | "ibmdos" | "ibm dos" => Ok(OsProduct::PcDos),
            "drdos" | "dr-dos" | "dr dos" => Ok(OsProduct::DrDos),
            "freedos" | "free-dos" | "free dos" => Ok(OsProduct::FreeDOS),
            _ => Err(OsError::UnsupportedOs),
        }
    }
}
