use crate::ManifestError;
use std::path::PathBuf;
use tempfile::tempdir;
use url::Url;
use LayerType::*;

#[derive(PartialEq)]
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
        if VALID_CATEGORIES.contains(&normalized_type.as_str()) {
            self.disk_type = Some(normalized_type);
            return Ok(());
        }
        Err(ManifestError::InvalidDiskType)
    }

    /// Download this layer's source file and stage its contents.
    pub fn download(&mut self) -> Result<(), ManifestError> {
        if self.layer_type == Software {
            if let Some(download_url) = &self.url {
                let staging_path = match download_url.scheme() {
                    "http" | "https" => self.download_http()?,
                    "ftp" => self.download_ftp()?,
                    // We have a URL, but we don't support the scheme type (yet).
                    _ => return Err(ManifestError::UnsupportedUrlScheme),
                };
                self.staging_path = Some(staging_path);
            } else {
                // We have the correct type of layer, but no URL is present.
                return Err(ManifestError::MissingUrl);
            }
        } else {
            // Downloading is only relevant on layers of type Software
            return Err(ManifestError::InvalidLayerType);
        }
        Ok(())
    }

    /// Download the Layer's url over HTTP(S)
    fn download_http(&mut self) -> Result<PathBuf, ManifestError> {
        let download_path = tempdir();

        if let Some(url) = &self.url {
            // Extract the file name from the URL's path.
            let path = url.path();
            let file_name = path.split('/').last().ok_or(ManifestError::InvalidUrl)?;
            if file_name.is_empty() {
                return Err(ManifestError::InvalidUrl);
            }
        }
        Err(ManifestError::InvalidUrl)
    }

    /// Download the Layer's url over FTP
    fn download_ftp(&mut self) -> Result<PathBuf, ManifestError> {
        println!("[TODO] Downloading over FTP.");
        Ok(PathBuf::new())
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
