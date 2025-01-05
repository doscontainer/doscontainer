mod error;
mod layer;

use std::{
    fs::File,
    io::Read,
    path::{Path, PathBuf},
};

use downloader::Downloader;
use error::CoreError;
use manifest::Manifest;

#[derive(Debug)]
pub struct DosContainer {
    manifest: Manifest,
    zipfiles: Vec<PathBuf>,
}

impl DosContainer {
    pub fn new(manifest: &Path) -> Result<Self, std::io::Error> {
        Ok(DosContainer {
            manifest: Manifest::load(manifest)?,
            zipfiles: Vec::new(),
        })
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// Download all the ZIP files that the layers specify
    pub fn download_layers(&mut self) -> Result<(), CoreError> {
        for layer in &self.manifest.layers {
            let downloader = Downloader::new(layer.url()).map_err(|_| CoreError::DownloadError)?;
            if let Some(checksum) = layer.checksum() {
                if !self.verify_checksum(downloader.zipfile(), checksum) {
                    return Err(CoreError::DownloadError);
                }
            }
            self.zipfiles.push(downloader.zipfile().to_path_buf());
        }
        Ok(())
    }

    fn verify_checksum(&self, file: &Path, checksum: &str) -> bool {
        // Open the file
        let mut file = File::open(&file)?;

        // Create a SHA256 hasher
        let mut hasher = Sha256::new();

        // Read the file in chunks to avoid high memory usage for large files
        let mut buffer = [0; 4096];
        loop {
            let bytes_read = file.read(&mut buffer)?;
            if bytes_read == 0 {
                break; // EOF
            }
            hasher.update(&buffer[..bytes_read]);
        }

        // Compute the hash and convert it to a hex string
        let computed_hash = format!("{:x}", hasher.finalize());

        // Compare the computed hash with the expected checksum
        if computed_hash == expected_checksum {
            true
        } else {
            false
        }
    }
}
