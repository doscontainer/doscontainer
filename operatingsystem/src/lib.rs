use std::str::FromStr;

use error::OsError;
use product::OsProduct;
use serde::Deserialize;
use url::Url;
use vendor::OsVendor;
use version::OsVersion;

pub mod bpb;
pub mod error;
pub mod product;
pub mod vendor;
pub mod version;

/// The OperatingSystem enum holds specific fragments of
/// code and data that apply only to a particular operating system
pub struct OperatingSystem {
    bootsector: [u8; 512],
    jumpcode: [u8; 3],
    msdossys: String,
    iosys: String,
    product: OsProduct,
    shortname: OsShortName,
    url: Url,
    vendor: OsVendor,
    version: OsVersion,
}

#[derive(Clone,Copy,Debug)]
pub enum OsShortName {
    IBMDOS100,
    IBMDOS110,
    IBMDOS200,
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

impl<'de> Deserialize<'de> for OsVersion {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let s: String = Deserialize::deserialize(deserializer)?;
        OsVersion::from_str(&s).map_err(|e| D::Error::custom(e.to_string()))
    }
}

impl OperatingSystem {
    /// Constructs a specific `OperatingSystem` instance from a vendor and version string.
    ///
    /// This method attempts to match the provided vendor and version against known supported
    /// DOS variants. If a matching combination is found, it returns a fully initialized
    /// `OperatingSystem` struct.
    ///
    /// # Arguments
    ///
    /// * `vendor` - A string representing the OS vendor (e.g., `"IBM"`).
    /// * `version` - A string representing the OS version (e.g., `"1.00"`).
    ///
    /// # Errors
    ///
    /// Returns [`OsError::InvalidOsVendor`] or [`OsError::InvalidOsVersionFormat`] if the inputs
    /// can't be parsed, or [`OsError::UnsupportedOs`] if the combination is not recognized.
    ///
    /// # Example
    ///
    /// ```rust
    /// let os = OperatingSystem::from_vendor_version("IBM", "1.00")?;
    /// assert_eq!(os.jumpcode(), [0xEB, 0x2F, 0x14]);
    /// ```
    ///
    /// [`OsError::InvalidOsVendor`]: crate::error::OsError::InvalidOsVendor
    /// [`OsError::InvalidOsVersionFormat`]: crate::error::OsError::InvalidOsVersionFormat
    /// [`OsError::UnsupportedOs`]: crate::error::OsError::UnsupportedOs
    pub fn from_vendor_version(vendor: &str, version: &str) -> Result<Self, OsError> {
        let vendor = OsVendor::from_str(vendor)?;
        let version = OsVersion::from_str(version)?;

        match (vendor, version) {
            // IBM PC-DOS 1.00
            (OsVendor::IBM, v) if v == OsVersion::new(1, 0) => Ok(Self {
                bootsector: *include_bytes!("bootsectors/pcdos-100.bin"),
                iosys: "IBMBIO.COM".to_string(),
                msdossys: "IBMDOS.COM".to_string(),
                product: OsProduct::PcDos,
                shortname: OsShortName::IBMDOS100,
                url: Url::from_str("https://dosk8s-dist.area536.com/ibm-pc-dos-100-bootstrap.zip")
                    .map_err(|_| OsError::InvalidUrl)?,
                vendor,
                version,
                jumpcode: [0xEB, 0x2F, 0x14],
            }),
            // IBM PC-DOS 1.10
            (OsVendor::IBM, v) if v == OsVersion::new(1, 10) => Ok(Self {
                bootsector: *include_bytes!("bootsectors/pcdos-110.bin"),
                iosys: "IBMBIO.COM".to_string(),
                msdossys: "IBMDOS.COM".to_string(),
                product: OsProduct::PcDos,
                shortname: OsShortName::IBMDOS110,
                url: Url::from_str("https://dosk8s-dist.area536.com/ibm-pc-dos-110-bootstrap.zip")
                    .map_err(|_| OsError::InvalidUrl)?,
                vendor,
                version,
                jumpcode: [0xEB, 0x27, 0x90],
            }),
            // IBM PC-DOS 2.00
            (OsVendor::IBM, v) if v == OsVersion::new(2, 0) => Ok(Self {
                bootsector: *include_bytes!("bootsectors/pcdos-200.bin"),
                iosys: "IBMBIO.COM".to_string(),
                msdossys: "IBMDOS.COM".to_string(),
                product: OsProduct::PcDos,
                shortname: OsShortName::IBMDOS200,
                url: Url::from_str("https://dosk8s-dist.area536.com/ibm-pc-dos-200-bootstrap.zip")
                    .map_err(|_| OsError::InvalidUrl)?,
                vendor,
                version,
                jumpcode: [0xEB, 0x27, 0x90],
            }),
            _ => Err(OsError::UnsupportedOs),
        }
    }

    /// Retrieve the jump code at the start of the boot sector
    pub fn jumpcode(&self) -> &[u8; 3] {
        &self.jumpcode
    }

    /// Return the filename this OS uses for the MSDOS.SYS equivalent system file.
    pub fn msdossys(&self) -> &str {
        self.msdossys.as_str()
    }

    /// Return the filename this OS uses for the IO.SYS equivalent system file.
    pub fn iosys(&self) -> &str {
        self.iosys.as_str()
    }

    /// Return the default download URL for an operating system zipfile
    pub fn download_url(&self) -> &str {
        self.url.as_str()
    }

    /// Return the filename this OS uses for the COMMAND.COM equivalent system file.
    pub fn commandcom(&self) -> String {
        "COMMAND.COM".to_string()
    }

    /// Returns the full boot sector / volume boot record for the OS variant specified
    pub fn bootsector(&self) -> &[u8; 512] {
        &self.bootsector
    }

    /// Return the ShortName field for easy matching
    pub fn shortname(&self) -> OsShortName {
        self.shortname
    }
}
