use config::{Config, File, FileFormat};
use error::ManifestError;
use serde::Deserialize;

use crate::layer::Layer;
use std::{collections::HashMap, path::Path};

mod error;
mod layer;
mod tests;

#[derive(Deserialize)]
pub struct Manifest {
    version: u32,
    layers: HashMap<String, Layer>,
}

impl Manifest {
    pub fn version(&self) -> u32 {
        self.version
    }

    pub fn set_version(&mut self, version: u32) {
        self.version = version;
    }

    pub fn insert_layer(&mut self, name: &str, layer: Layer) {
        self.layers.insert(String::from(name), layer);
    }

    pub fn layer(&self, name: &str) -> Option<&Layer> {
        self.layers.get(name)
    }

    pub fn mut_layer(&mut self, name: &str) -> Option<&mut Layer> {
        self.layers.get_mut(name)
    }

    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, ManifestError> {
        let settings = Config::builder()
            .add_source(File::from(path.as_ref()).format(FileFormat::Toml))
            .build()
            .map_err(ManifestError::ConfigBuild)?;

        settings
            .try_deserialize::<Manifest>()
            .map_err(ManifestError::Deserialize)
    }
}

impl Default for Manifest {
    fn default() -> Manifest {
        Manifest {
            version: 1,
            layers: HashMap::new(),
        }
    }
}
