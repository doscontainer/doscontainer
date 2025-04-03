use std::collections::HashMap;

pub struct Manifest<V> {
    version: usize,
    layers: Vec<Layer<V>>,
}

pub struct Layer<V> {
    name: String,
    fields: HashMap<String, V>,
}

impl<V> Layer<V> {
    pub fn new(name: Option<&str>) -> Self {
        Layer {
            name: name.unwrap_or_default().to_string(),
            fields: HashMap::new(),
        }
    }
}
