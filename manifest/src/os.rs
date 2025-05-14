use std::{fmt, str::FromStr};

use serde::Deserialize;

use crate::error::ManifestError;

#[derive(Debug)]
pub enum OsVendor {
    DigitalResearch,
    IBM,
    Microsoft,
}

#[derive(Debug)]
pub struct OperatingSystem {
    vendor: OsVendor,
    version: OsVersion,
}

impl OsVendor {
    pub fn from_product(product: &str) -> Result<Self, ManifestError> {
        match product.trim().to_lowercase().as_str() {
            "ms-dos" | "msdos" | "dos" => Ok(OsVendor::Microsoft),
            "pc-dos" | "pcdos" | "ibmdos" => Ok(OsVendor::IBM),
            "dr-dos" | "drdos" => Ok(OsVendor::DigitalResearch),
            _ => Err(ManifestError::InvalidOsProduct(product.to_string())),
        }
    }
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let osname = match self.vendor {
            OsVendor::DigitalResearch => "DR-DOS",
            OsVendor::IBM => "PC-DOS",
            OsVendor::Microsoft => "MS-DOS",
        };
        Ok(write!(f, "{} {} {}", self.vendor, osname, self.version)?)
    }
}

impl Default for OperatingSystem {
    fn default() -> Self {
        OperatingSystem {
            vendor: OsVendor::Microsoft,
            version: OsVersion {
                major: 6,
                minor: 22,
            },
        }
    }
}

#[derive(Debug, Deserialize, Clone, Copy, Eq, PartialEq, Ord, PartialOrd)]
pub struct OsVersion {
    pub major: u8,
    pub minor: u8,
}

impl fmt::Display for OsVersion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Ok(write!(f, "{}.{}", self.major, self.minor)?)
    }
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
            OsVendor::DigitalResearch => Ok(write!(f, "Digital Research")?),
            OsVendor::IBM => Ok(write!(f, "IBM")?),
            OsVendor::Microsoft => Ok(write!(f, "Microsoft")?),
        }
    }
}

impl FromStr for OsVendor {
    type Err = ManifestError;

    fn from_str(input: &str) -> Result<Self, ManifestError> {
        match input.trim().to_lowercase().as_str() {
            "dr" | "dri" | "digital" | "digital research" | "digitalresearch" => {
                Ok(OsVendor::DigitalResearch)
            }
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

impl FromStr for OperatingSystem {
    type Err = ManifestError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.trim().splitn(2, ' ');

        let vendor_str = parts.next().ok_or(ManifestError::MissingVendor)?;
        let version_str = parts.next().ok_or(ManifestError::MissingVersion)?;

        let vendor = OsVendor::from_product(vendor_str)?;
        let version = OsVersion::from_str(version_str)?;

        Ok(OperatingSystem { vendor, version })
    }
}