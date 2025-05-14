use std::{fmt, str::FromStr};

use serde::Deserialize;

use crate::error::ManifestError;

#[derive(Debug)]
pub enum OsVendor {
    IBM,
    Microsoft,
}

#[derive(Debug)]
pub struct OperatingSystem {
    vendor: OsVendor,
    version: OsVersion,
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct OsVersion {
    pub major: u8,
    pub minor: u8,
}

impl FromStr for OsVersion {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.trim().split('.').collect();
        if parts.len() != 2 {
            return Err(ManifestError::InvalidOsVersionFormat(s.to_string()));
        }

        let major = parts[0]
            .parse::<u8>()
            .map_err(|e| ManifestError::InvalidOsVersionFormat(e.to_string()))?;
        let minor = parts[1]
            .parse::<u8>()
            .map_err(|e| ManifestError::InvalidOsVersionFormat(e.to_string()))?;

        Ok(OsVersion { major, minor })
    }
}

impl fmt::Display for OsVendor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OsVendor::IBM => Ok(write!(f, "IBM")?),
            OsVendor::Microsoft => Ok(write!(f, "Microsoft")?),
        }
    }
}

impl FromStr for OsVendor {
    type Err = ManifestError;

    fn from_str(input: &str) -> Result<Self, ManifestError> {
        match input.trim().to_lowercase().as_str() {
            "ibm" => Ok(OsVendor::IBM),
            "ms" | "microsoft" | "micro-soft" | "micro soft" => Ok(OsVendor::Microsoft),
            _ => Err(ManifestError::InvalidOsVendor(input.to_string())),
        }
    }
}

impl<'de> Deserialize<'de> for OsVendor {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let s: String = Deserialize::deserialize(deserializer)?;
        OsVendor::from_str(&s).map_err(|e| D::Error::custom(e.to_string()))
    }
}
