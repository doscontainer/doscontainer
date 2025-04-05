use crate::fields::{FieldRef, FieldValue};
use std::collections::HashMap;

/// Manifests consist primary of layers
#[derive(Debug)]
pub struct Layer {
    fields: HashMap<String, FieldValue>,
}

impl Layer {
    pub fn new() -> Self {
        Layer {
            fields: HashMap::new(),
        }
    }

    pub fn insert_field(&mut self, key: impl Into<String>, value: FieldValue) {
        self.fields.insert(key.into(), value);
    }

    pub fn field(&self, key: &str) -> FieldRef<'_> {
        FieldRef(self.fields.get(key))
    }
}

impl Default for Layer {
    fn default() -> Self {
        Layer::new()
    }
}
