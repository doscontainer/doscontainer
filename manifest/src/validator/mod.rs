use crate::validator::error::ValidationError;
use crate::Manifest;

pub struct Validator {}

mod error;
mod tests;

impl Validator {
    pub fn validate(manifest: &Manifest) -> Result<(), ValidationError> {
        if !Self::required_layers(manifest) {
            return Err(ValidationError::RequiredLayerMissing);
        }
        if !Self::physical_required_fields(manifest) {
            return Err(ValidationError::PhysicalFieldsMissing);
        }
        Ok(())
    }

    /// Ensure the required layers are present in the manifest
    fn required_layers(manifest: &Manifest) -> bool {
        if manifest.layer("foundation").is_none() {
            return false;
        }
        if manifest.layer("physical").is_none() {
            return false;
        }
        true
    }

    fn physical_required_fields(manifest: &Manifest) -> bool {
        let layer = manifest.layer("physical");
        if layer.unwrap().field("category").0.is_none() {
            return false;
        }
        true
    }
}
