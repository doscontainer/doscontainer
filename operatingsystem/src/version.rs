use std::{fmt, str::FromStr};

use crate::error::OsError;

/// Represents a DOS version as a `major.minor` pair (e.g., `1.10`).
///
/// This struct is used to express version compatibility constraints in
/// manifests, hardware specs, and internal logic. It supports ordering,
/// equality, parsing from string, and formatting.
#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq, Ord, PartialOrd)]
pub struct OsVersion {
    major: u8,
    minor: u8,
}

impl OsVersion {
    pub const fn new(major: u8, minor: u8) -> Self {
        Self { major, minor }
    }
}

impl fmt::Display for OsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

impl FromStr for OsVersion {
    type Err = OsError;

    /// Parses a version string like `"1.10"` or `"2"` into an `OsVersion`.
    ///
    /// If only a major version is provided (e.g., `"2"`), the minor
    /// version defaults to `0`. Returns an error if the format is invalid
    /// or if parsing fails.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().split('.');
        let major = parts
            .next()
            .ok_or_else(|| OsError::InvalidOsVersionFormat(s.to_string()))?
            .parse::<u8>()
            .map_err(|e| OsError::InvalidOsVersionFormat(e.to_string()))?;

        let minor = parts
            .next()
            .map(|p| {
                p.parse::<u8>()
                    .map_err(|e| OsError::InvalidOsVersionFormat(e.to_string()))
            })
            .transpose()?
            .unwrap_or(0); // Default to .0 if omitted

        if parts.next().is_some() {
            return Err(OsError::InvalidOsVersionFormat(s.to_string()));
        }

        Ok(OsVersion { major, minor })
    }
}
