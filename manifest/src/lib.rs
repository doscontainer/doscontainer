use std::collections::HashMap;

/// The FieldValue is a way for Layer to support heterogenous values in its
/// without the need for a generic. We only need a small number of value
/// types anyway, so this enum facilitates that.
#[derive(Clone, Debug)]
pub enum FieldValue {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

#[derive(Debug)]
pub struct FieldRef<'a>(pub Option<&'a FieldValue>);

impl PartialEq for FieldValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (FieldValue::String(a), FieldValue::String(b)) => a == b,
            (FieldValue::Integer(a), FieldValue::Integer(b)) => a == b,
            (FieldValue::Float(a), FieldValue::Float(b)) => a == b,
            (FieldValue::Boolean(a), FieldValue::Boolean(b)) => a == b,
            _ => false,
        }
    }
}

impl PartialEq<str> for FieldValue {
    fn eq(&self, other: &str) -> bool {
        matches!(self, FieldValue::String(s) if s == other)
    }
}

impl PartialEq<String> for FieldValue {
    fn eq(&self, other: &String) -> bool {
        matches!(self, FieldValue::String(s) if s == other)
    }
}

impl PartialEq<i64> for FieldValue {
    fn eq(&self, other: &i64) -> bool {
        matches!(self, FieldValue::Integer(i) if i == other)
    }
}

impl PartialEq<f64> for FieldValue {
    fn eq(&self, other: &f64) -> bool {
        matches!(self, FieldValue::Float(f) if f == other)
    }
}

impl PartialEq<bool> for FieldValue {
    fn eq(&self, other: &bool) -> bool {
        matches!(self, FieldValue::Boolean(b) if b == other)
    }
}

impl PartialEq<&str> for FieldRef<'_> {
    fn eq(&self, other: &&str) -> bool {
        matches!(self.0, Some(FieldValue::String(s)) if s == other)
    }
}

impl PartialEq<String> for FieldRef<'_> {
    fn eq(&self, other: &String) -> bool {
        matches!(self.0, Some(FieldValue::String(s)) if s == other)
    }
}

impl PartialEq<i64> for FieldRef<'_> {
    fn eq(&self, other: &i64) -> bool {
        matches!(self.0, Some(FieldValue::Integer(i)) if i == other)
    }
}

impl PartialEq<f64> for FieldRef<'_> {
    fn eq(&self, other: &f64) -> bool {
        matches!(self.0, Some(FieldValue::Float(f)) if f == other)
    }
}

impl PartialEq<bool> for FieldRef<'_> {
    fn eq(&self, other: &bool) -> bool {
        matches!(self.0, Some(FieldValue::Boolean(b)) if b == other)
    }
}
pub struct Manifest {
    version: usize,
    layers: Vec<Layer>,
}

pub struct Layer {
    name: String,
    fields: HashMap<String, FieldValue>,
}

impl Layer {
    pub fn new(name: impl Into<String>) -> Self {
        Layer {
            name: name.into(),
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

impl Manifest {
    pub fn new() -> Self {
        Manifest {
            version: 1,
            layers: Vec::new(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn str_field() {
        let mut layer = Layer::new("foundation");
        layer.insert_field("os", FieldValue::String(String::from("test")));
        assert_eq!(layer.field("os"), "test");
    }

    #[test]
    fn string_field() {
        let mut layer = Layer::new("foundation");
        layer.insert_field("os", FieldValue::String(String::from("test")));
        assert_eq!(layer.field("os"), String::from("test"));
    }

    #[test]
    fn int_field() {
        let mut layer = Layer::new("foundation");
        layer.insert_field("os", FieldValue::Integer(5));
        assert_eq!(layer.field("os"), 5);
    }

    #[test]
    fn float_field() {
        let mut layer = Layer::new("foundation");
        layer.insert_field("os", FieldValue::Float(3.1415952));
        assert_eq!(layer.field("os"), 3.1415952);
    }

    #[test]
    fn bool_field() {
        let mut layer = Layer::new("foundation");
        layer.insert_field("os", FieldValue::Boolean(false));
        assert_eq!(layer.field("os"), false);
    }
}
