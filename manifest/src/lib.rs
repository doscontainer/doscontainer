use crate::layer::Layer;
use std::collections::HashMap;

mod layer;
mod tests;

pub enum ManifestError {
    InvalidDiskCategory,
    InvalidDiskType,
    InvalidUrl,
}

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
}

impl Default for Manifest {
    fn default() -> Manifest {
        Manifest {
            version: 1,
            layers: HashMap::new(),
        }
    }
}
