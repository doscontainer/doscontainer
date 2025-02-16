use std::fmt;

use serde::{Deserialize, Serialize};

use crate::{error::ManifestError, layer::Layer};

#[derive(Debug, Deserialize, Serialize)]
pub struct OperatingSystem {
    pub version: String,
    #[serde(default = "default_osvariant")]
    pub variant: String,
}

pub fn default_osvariant() -> String {
    "minimal".to_string()
}

impl fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "OS Version       : {}\nVariant          : {}\n",
            self.display_version(),
            self.display_variant()
        )
    }
}

impl OperatingSystem {
    pub fn display_version(&self) -> String {
        match self.version.as_str() {
            "IBMDOS100" => "IBM PC-DOS 1.00".to_string(),
            "IBMDOS110" => "IBM PC-DOS 1.10".to_string(),
            "IBMDOS200" => "IBM PC-DOS 2.00".to_string(),
            _ => self.version.clone(), // Fallback to the original version if no match
        }
    }

    pub fn display_variant(&self) -> String {
        match self.variant.as_str() {
            "minimal" => "Minimal installation.".to_string(),
            "fullinstall" => "Full installation".to_string(),
            _ => self.version.clone(),
        }
    }

    /// Converts the OS to a Layer so we can download the assets
    pub fn as_layer(&self) -> Result<Layer, ManifestError> {
        match self.version.as_str() {
            "IBMDOS100" => Ok(Layer::new(
                "https://dosk8s-dist.area536.com/ibm-pc-dos-100-bootstrap.zip",
                Some("IBM PC-DOS 1.00"),
                Some("fb2bd093c3d9019e07711ef9202ac6299dc697932aef47b2b2d7ce5926be9118"),
            )),
            "IBMDOS110" => Ok(Layer::new(
                "https://dosk8s-dist.area536.com/ibm-pc-dos-110-bootstrap.zip",
                Some("IBM PC-DOS 1.10"),
                Some("feb7d0854312a96af6a94b469ad42f907d71ff695b30f742379f810aa73e6acd"),
            )),
            _ => Err(ManifestError::UnsupportedOperatingSystem),
        }
    }

    pub fn version(&self) -> &str {
        self.version.as_str()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OsSettings {
    full_install: Option<bool>,
}

impl fmt::Display for OsSettings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Full Install     : {}\n",
            self.full_install.unwrap_or(false)
        )
    }
}
