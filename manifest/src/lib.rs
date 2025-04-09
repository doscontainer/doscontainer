use layer::Layer;
use std::collections::HashMap;

pub mod fields;
pub mod layer;
pub mod validator;

/// A Manifest is the composed whole of configuration for a single game or
/// application's build instructions. It consists of a version and any
/// number of layers.
///
/// In order for a Manifest to be entirely valid, you need at least the
/// following layers:
///   - physical
///   - foundation
#[derive(Debug)]
pub struct Manifest {
    version: usize,
    layers: HashMap<String, Layer>,
}

impl Manifest {
    pub fn new() -> Self {
        Manifest {
            version: 1,
            layers: HashMap::new(),
        }
    }

    pub fn version(&self) -> usize {
        self.version
    }

    pub fn insert_layer(&mut self, name: &str, layer: Layer) {
        self.layers.insert(String::from(name), layer);
    }

    pub fn layer(&self, name: &str) -> Option<&Layer> {
        self.layers.get(name)
    }

    pub fn layers(&self) -> &HashMap<String, Layer> {
        &self.layers
    }
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest::new()
    }
}
