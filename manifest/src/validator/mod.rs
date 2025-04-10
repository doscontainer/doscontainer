use crate::validator::error::ValidationError;
use crate::{layer, Manifest};

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
        for item in ["foundation", "physical"] {
            if manifest.layer(item).is_none() {
                return false;
            }
        }
        true
    }

    fn physical_required_fields(manifest: &Manifest) -> bool {
        let layer = manifest.layer("physical");
        for item in ["category", "type"] {
            if layer.unwrap().field("item").0.is_none() {
                return false;
            }
        }
        true
    }

}
