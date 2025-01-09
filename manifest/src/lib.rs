mod disk;
mod error;
mod gameconfig;
mod gamemetadata;
mod layer;
pub mod operatingsystem;

use disk::Disk;
use gamemetadata::GameMetadata;
use layer::Layer;
use operatingsystem::OperatingSystem;
use serde::{Deserialize, Serialize};
use std::{
    fmt,
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

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
