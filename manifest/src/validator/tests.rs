#[cfg(test)]
mod tests {
    use crate::validator::{ValidationError, Validator};
    use crate::{Layer, Manifest};

    /// We do not have all of the required layers
    #[test]
    fn required_layers() {
        let manifest = Manifest::new();
        assert_eq!(
            Err(ValidationError::RequiredLayerMissing),
            Validator::validate(&manifest)
        );
    }

    /// The layers are present, but the fields are not.
    #[test]
    fn required_layers_present() {
        let mut manifest = Manifest::new();
        manifest.insert_layer("physical", Layer::new());
        manifest.insert_layer("foundation", Layer::new());
        assert_eq!(
            Err(ValidationError::PhysicalFieldsMissing),
            Validator::validate(&manifest)
        );
    }
}
