use crate::ManifestError;
use url::Url;
use LayerType::*;

pub enum LayerType {
    Foundation,
    Physical,
    Software,
}

pub struct Layer {
    layer_type: LayerType,
    url: Option<Url>,
    disk_category: Option<String>,
    disk_type: Option<String>,
    filesystem: Option<String>,
    cylinders: Option<usize>,
    heads: Option<usize>,
    sectors: Option<usize>,
}

impl Layer {
    pub fn set_url(&mut self, url: &str) -> Result<(), ManifestError> {
        match Url::parse(url) {
            Ok(_) => Ok(()),
            Err(url) => Err(ManifestError::InvalidUrl),
        }
    }
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            layer_type: Software,
            url: None,
            disk_category: None,
            disk_type: None,
            filesystem: None,
            cylinders: None,
            heads: None,
            sectors: None,
        }
    }
}
