mod error;

use error::ManifestError;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct GameMetadata {
    title: String,
    publisher: String,
    year: u32,
    comment: Option<String>,
}

impl fmt::Display for GameMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Title            : {}\nPublisher        : {}\nYear             : {}\nComment          : {}\n",
            self.title, self.publisher, self.year, self.comment.as_deref().unwrap_or("N/A")
        )
    }
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Layer {
    url: String,
    checksum: Option<String>,
    label: Option<String>,
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "URL              : {}\nLabel            : {}\nChecksum         : {}\n",
            self.url,
            self.label.as_deref().unwrap_or("N/A"),
            self.checksum.as_deref().unwrap_or("N/A")
        )
    }
}

impl Layer {
    pub fn url(&self) -> &str {
        &self.url
    }

    pub fn label(&self) -> String {
        if self.label.is_some() {
            self.label.clone().unwrap()
        } else {
            self.url.to_owned()
        }
    }

    pub fn checksum(&self) -> Option<&str> {
        self.checksum.as_deref()
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct GameConfig {
    autoexec: String,
    config: String,
}

impl fmt::Display for GameConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Autoexec         : {}\nConfig           : {}\n",
            self.autoexec, self.config
        )
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Geometry {
    pub cylinders: usize,
    pub heads: usize,
    pub sectors: usize,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Disk {
    pub name: PathBuf,
    pub label: Option<String>,
    pub size: Option<u32>,
    #[serde(default = "default_filesystem")]
    pub filesystem: String,
    #[serde(default = "default_disktype")]
    pub disktype: String,
    #[serde(default = "default_harddisktype")]
    pub harddisktype: String,
    pub geometry: Option<Geometry>,
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Disk Name        : {:?}\nLabel            : {}\nSize (MB)        : {}\nDisk Type        : {}\n",
            self.name,
            self.label.as_deref().unwrap_or("N/A"),
            self.size.unwrap_or(0),
            self.display_disktype()
        )
    }
}

impl Disk {
    pub fn display_disktype(&self) -> String {
        match self.disktype.to_ascii_lowercase().as_str() {
            "ide" => "Hard disk: modern IDE/ATA.".to_string(),
            "pcxt" => "Hard disk: IBM XT compatible".to_string(),
            "pcat" => "Hard disk: IBM AT compatible".to_string(),
            "f35_720" => "Floppy: 3.5\" DD 720KB".to_string(),
            "f35_1440" => "Floppy: 3.5\" HD 1.44MB".to_string(),
            "f35_2880" => "Floppy: 3.5\" XD 2.88MB".to_string(),
            "f525_160" => "Floppy: 5.25\" 160KB".to_string(),
            "f525_180" => "Floppy: 5.25\" 180KB".to_string(),
            "f525_320" => "Floppy: 5.25\" 320KB".to_string(),
            "f525_360" => "Floppy: 5.25\" 360KB".to_string(),
            "f525_1200" => "Floppy: 5.25\" 1.2MB".to_string(),
            _ => "Unknown disk type. Please report this as a bug!".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OperatingSystem {
    pub version: String,
    #[serde(default = "default_osvariant")]
    pub variant: String,
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
            "IBMDOS100" => Ok(Self::ibmdos_100_layer()),
            "IBMDOS110" => Ok(Self::ibmdos_110_layer()),
            _ => Err(ManifestError::UnsupportedOperatingSystem),
        }
    }

    /// Construct a Layer for IBM PC-DOS 1.00
    fn ibmdos_100_layer() -> Layer {
        Layer {
            url: "https://dosk8s-dist.area536.com/ibm-pc-dos-100-bootstrap.zip".to_string(),
            label: Some("IBM PC-DOS 1.00".to_string()),
            checksum: Some(
                "fb2bd093c3d9019e07711ef9202ac6299dc697932aef47b2b2d7ce5926be9118".to_string(),
            ),
        }
    }

    /// Construct a Layer for IBM PC-DOS 1.10
    fn ibmdos_110_layer() -> Layer {
        Layer {
            url: "https://dosk8s-dist.area536.com/ibm-pc-dos-110-bootstrap.zip".to_string(),
            label: Some("IBM PC-DOS 1.10".to_string()),
            checksum: Some(
                "feb7d0854312a96af6a94b469ad42f907d71ff695b30f742379f810aa73e6acd".to_string(),
            ),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Manifest {
    pub version: Option<u32>,
    pub metadata: GameMetadata,
    pub layers: Vec<Layer>,
    pub disk: Disk,
    pub os: OperatingSystem,
    #[serde(rename = "AUTOEXEC")]
    pub autoexec: Option<String>,
    #[serde(rename = "CONFIG")]
    pub config: Option<String>,
}

fn default_disktype() -> String {
    "HardDisk".to_string()
}

fn default_filesystem() -> String {
    "FAT12".to_string()
}

fn default_harddisktype() -> String {
    "CUSTOM".to_string()
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Manifest Version : {}\n", self.version.unwrap_or(0))?;
        write!(f, "{}", self.metadata)?;
        write!(f, "Layers           :\n")?;
        for (i, layer) in self.layers.iter().enumerate() {
            write!(f, "Layer {}:\n{}", i + 1, layer)?;
        }
        write!(f, "{}", self.disk)?;
        write!(f, "{}", self.os)?;
        write!(
            f,
            "AUTOEXEC         : {}\n",
            self.autoexec.as_deref().unwrap_or("N/A")
        )?;
        write!(
            f,
            "CONFIG           : {}\n",
            self.config.as_deref().unwrap_or("N/A")
        )?;
        Ok(())
    }
}

impl Manifest {
    pub fn load(yaml_path: &Path) -> Result<Manifest, std::io::Error> {
        let mut file = File::open(yaml_path)?;
        let mut yaml = String::new();
        file.read_to_string(&mut yaml)?;
        let mut manifest: Manifest =
            serde_yaml::from_str(&yaml).expect("Failed to convert YAML to Manifest.");
        manifest.layers.push(manifest.os.as_layer().unwrap());
        Ok(manifest)
    }

    pub fn metadata(&self) -> &GameMetadata {
        &self.metadata
    }

    pub fn default_disktype() -> String {
        "ide".to_string()
    }

    pub fn disktype(&self) -> &str {
        &self.disk.disktype
    }

    pub fn diskfile(&self) -> PathBuf {
        self.disk.name.clone()
    }

    pub fn operating_system(&self) -> &OperatingSystem {
        &self.os
    }
}

pub fn default_osvariant() -> String {
    "minimal".to_string()
}
