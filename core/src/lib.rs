use std::path::Path;

use manifest::Manifest;

#[derive(Debug)]
pub struct DosContainer {
    manifest: Manifest,
}

impl DosContainer {
    pub fn new(manifest: &Path) -> Result<Self, std::io::Error> {
        Ok(DosContainer {
            manifest: Manifest::load(manifest)?,
        })
    }
}