use std::collections::HashMap;
use layer::Layer;

pub mod fields;
pub mod layer;

/// The base manifest struct
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
}

impl Default for Manifest {
    fn default() -> Self {
        Manifest::new()
    }
}
