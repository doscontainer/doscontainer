use crate::ManifestError;
use std::path::PathBuf;
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
    staging_path: Option<PathBuf>,
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

    pub fn url(&self) -> &Option<Url> {
        &self.url
    }

    pub fn set_disk_category(&mut self, category: &str) -> Result<(), ManifestError> {
        const VALID_CATEGORIES: [&str; 2] = ["FLOPPY", "HDD"];

        let normalized_category = category.to_ascii_uppercase();
        if VALID_CATEGORIES.contains(&normalized_category.as_str()) {
            self.disk_category = Some(normalized_category);
            return Ok(());
        }
        Err(ManifestError::InvalidDiskCategory)
    }

    pub fn set_disk_type(&mut self, disktype: &str) -> Result<(), ManifestError> {
        const VALID_CATEGORIES: [&str; 8] = [
            "F525_160", "F525_180", "F525_320", "F525_360", "F525_12M", "F35_720", "F35_144",
            "F35_288",
        ];

        let normalized_type = disktype.to_ascii_uppercase();
        if VALID_CATEGORIES.contains(&&normalized_type.as_str()) {
            self.disk_type = Some(normalized_type);
            return Ok(());
        }
        Err(ManifestError::InvalidDiskType)
    }
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            layer_type: Software,
            url: None,
            staging_path: None,
            disk_category: None,
            disk_type: None,
            filesystem: None,
            cylinders: None,
            heads: None,
            sectors: None,
        }
    }
}
