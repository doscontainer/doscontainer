use std::{fmt, str::FromStr};

use serde::Deserialize;

use crate::error::ManifestError;


#[derive(Debug)]
pub struct OperatingSystem {
    vendor: OsVendor,
    version: OsVersion,
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