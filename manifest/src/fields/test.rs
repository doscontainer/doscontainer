#[cfg(test)]
mod test {

    use crate::fields::FieldValue;
    use crate::Layer;

    #[test]
    fn str_field() {
        let mut layer = Layer::new();
        layer.insert_field("os", FieldValue::String(String::from("test")));
        assert_eq!(layer.field("os"), "test");
    }

    #[test]
    fn string_field() {
        let mut layer = Layer::new();
        layer.insert_field("os", FieldValue::String(String::from("test")));
        assert_eq!(layer.field("os"), String::from("test"));
    }

    #[test]
    fn int_field() {
        let mut layer = Layer::new();
        layer.insert_field("os", FieldValue::Integer(5));
        assert_eq!(layer.field("os"), 5);
    }

    #[test]
    fn float_field() {
        let mut layer = Layer::new();
        layer.insert_field("os", FieldValue::Float(3.1415952));
        assert_eq!(layer.field("os"), 3.1415952);
    }

    #[test]
    fn bool_field() {
        let mut layer = Layer::new();
        layer.insert_field("os", FieldValue::Boolean(false));
        assert_eq!(layer.field("os"), false);
    }

    #[test]
    fn field_not_present() {
        let mut layer = Layer::new();
        assert!(layer.field("nosuchfield").is_absent());
    }
}
