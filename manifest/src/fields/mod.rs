mod test;

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

/// FieldRef is a wrapper type so that we can implement PartialEq for every
/// variant.
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
