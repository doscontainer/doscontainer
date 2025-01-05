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
use sha2::{Digest, Sha256};

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

    /// Downloads all the ZIP files specified by the layers in the manifest and verifies their checksums (if provided).
    ///
    /// # Description
    /// This function iterates through the layers defined in the manifest, downloads the ZIP files from the specified URLs,
    /// verifies their checksums if a checksum is provided, and stores the paths to the downloaded files in the `zipfiles` field.
    ///
    /// # Errors
    /// This function returns a `CoreError` in the following cases:
    /// - `CoreError::DownloadError`: If the downloader fails to initialize or download the file.
    /// - `CoreError::ChecksumError`: If a checksum is provided for a layer but it does not match the computed checksum of the file.
    /// - `CoreError`: If `verify_checksum` encounters an error while verifying the file's checksum.
    ///
    /// # Returns
    /// - `Ok(())`: If all layers are successfully downloaded and verified.
    /// - `Err(CoreError)`: If an error occurs during the download or verification process.
    pub fn download_layers(&mut self) -> Result<(), CoreError> {
        for layer in &self.manifest.layers {
            let downloader = Downloader::new(layer.url()).map_err(|_| CoreError::DownloadError)?;
            if let Some(checksum) = layer.checksum() {
                if !self.verify_layer_checksum(downloader.zipfile(), checksum)? {
                    return Err(CoreError::ChecksumError);
                }
            }
            self.zipfiles.push(downloader.zipfile().to_path_buf());
        }
        Ok(())
    }

    /// Verifies the SHA256 checksum of a given layer's zipfile.
    ///
    /// # Arguments
    ///
    /// * `file` - A reference to the path of the file to verify.
    /// * `checksum` - A string containing the expected SHA256 checksum in hexadecimal format.
    ///
    /// # Returns
    ///
    /// * `Ok(true)` if the file's checksum matches the provided checksum.
    /// * `Ok(false)` if the file's checksum does not match the provided checksum.
    /// * `Err(CoreError)` if an error occurs while reading the file.
    ///
    /// # Errors
    ///
    /// This function returns a `CoreError` in the following cases:
    /// - `CoreError::FileOpenError`: If the file cannot be opened.
    /// - `CoreError::FileReadError`: If an error occurs while reading the file.
    fn verify_layer_checksum(&self, file: &Path, checksum: &str) -> Result<bool, CoreError> {
        // Open the file
        let mut file = File::open(&file).map_err(|_| CoreError::DownloadError)?;

        // Create a SHA256 hasher
        let mut hasher = Sha256::new();

        // Read the file in chunks to avoid high memory usage for large files
        let mut buffer = [0; 4096];
        loop {
            let bytes_read = file
                .read(&mut buffer)
                .map_err(|_| CoreError::FileReadError)?;
            if bytes_read == 0 {
                break; // EOF
            }
            hasher.update(&buffer[..bytes_read]);
        }

        // Compute the hash and convert it to a hex string
        let computed_hash = format!("{:x}", hasher.finalize());

        // Compare the computed hash with the expected checksum
        Ok(computed_hash == checksum)
    }
}
