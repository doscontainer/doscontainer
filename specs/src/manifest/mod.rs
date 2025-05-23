use config::{Config, File, FileFormat};
use metadata::Metadata;

use crate::error::SpecError;
use layer::Layer;
use serde::Deserialize;


use std::{collections::HashMap, fmt, path::Path};

mod layer;
mod metadata;
mod storage;
mod tests;

#[derive(Deserialize)]
pub struct Manifest {
    version: u32,
    metadata: Option<Metadata>,
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

    pub fn layers(&self) -> &HashMap<String, Layer> {
        &self.layers
    }

    pub fn layer(&self, name: &str) -> Option<&Layer> {
        self.layers.get(name)
    }

    pub fn mut_layer(&mut self, name: &str) -> Option<&mut Layer> {
        self.layers.get_mut(name)
    }

    /// Loads a `Manifest` from a TOML file at the specified path.
    ///
    /// This function attempts to read a TOML file and deserialize its contents
    /// into a `Manifest` instance. It uses the `config` crate to handle parsing
    /// and supports proper error mapping for build and deserialization issues.
    ///
    /// # Type Parameters
    /// - `P`: A type that can be referenced as a `Path`, such as `&str` or `PathBuf`.
    ///
    /// # Arguments
    /// - `path`: The path to the TOML file to load.
    ///
    /// # Returns
    /// - `Ok(Manifest)`: If the file was successfully read and deserialized.
    /// - `Err(SpecError)`: If there was an error reading or deserializing the file.
    ///
    /// # Errors
    /// - Returns `SpecError::ConfigBuild` if the configuration builder fails.
    /// - Returns `SpecError::Deserialize` if deserialization into `Manifest` fails.
    pub fn from_toml<P: AsRef<Path>>(path: P) -> Result<Self, SpecError> {
        let settings = Config::builder()
            .add_source(File::from(path.as_ref()).format(FileFormat::Toml))
            .build()
            .map_err(SpecError::ConfigBuild)?;

        settings
            .try_deserialize::<Manifest>()
            .map_err(SpecError::Deserialize)
    }
}

impl Default for Manifest {
    fn default() -> Manifest {
        Manifest {
            version: 1,
            metadata: None,
            layers: HashMap::new(),
        }
    }
}

impl fmt::Display for Manifest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "DOSContainer build manifest")?;
        writeln!(f, "-----------------------------------")?;
        writeln!(f, " Manifest format version : {}", self.version())?;
        if let Some(metadata) = &self.metadata {
            writeln!(f, " Application             : {}", metadata.application)?;
            writeln!(f, " Developer               : {}", metadata.developer)?;
            writeln!(f, " Genres                  : {}", metadata.genres.join(", "))?;
            writeln!(f, " Year                    : {}", metadata.year)?;
        }
        for layer in self.layers() {
            writeln!(f, "{} {}", layer.0, layer.1)?;
        }
        Ok(())
    }
}
