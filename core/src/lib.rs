use std::path::Path;

#[derive(Debug)]
pub struct DosContainer {
}

impl DosContainer {
    pub fn new(manifest: &Path) -> Self {
        DosContainer{}
    }
}