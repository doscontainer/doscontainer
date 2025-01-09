mod error;

use std::{
    fs::File,
    io::{copy, BufReader, Read},
    path::Path,
};

use disk::{disktype::DiskType, floppy::Floppy, Disk};
use downloader::Downloader;
use error::CoreError;
use manifest::Manifest;
use sha2::{Digest, Sha256};
use tempfile::TempDir;
use zip::ZipArchive;

pub struct DosContainer {
    disk: Box<dyn Disk>,
    manifest: Manifest,
    staging_dir: TempDir,
}

impl DosContainer {
    pub fn new(manifest: &Path) -> Result<Self, CoreError> {
        let mut loaded_manifest = Manifest::load(manifest).map_err(|_| CoreError::DiskTypeError)?;
        let disktype =
            DiskType::new(loaded_manifest.disktype()).map_err(|_| CoreError::DiskTypeError)?;
        let disk = Floppy::new(disktype, loaded_manifest.diskfile())
            .map_err(|_| CoreError::CreateFileError)?;
        Ok(DosContainer {
            disk: Box::new(disk),
            manifest: loaded_manifest,
            staging_dir: TempDir::new().map_err(|_| CoreError::CreateDirError)?,
        })
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn staging_dir(&self) -> &Path {
        self.staging_dir.path()
    }

    /// Downloads and extracts layers defined in the manifest.
    ///
    /// This function iterates over each layer in the `manifest.layers` field, downloading the
    /// corresponding ZIP file, verifying its checksum (if provided), and extracting its contents
    /// into the specified `staging_dir`. It handles both files and directories within the ZIP
    /// archive and sets appropriate permissions on Unix-based systems.
    ///
    /// # Process:
    /// 1. Initializes a `Downloader` for each layer's URL.
    /// 2. Retrieves the ZIP file path from the downloader and opens the file.
    /// 3. Verifies the checksum (if provided) against the downloaded file.
    /// 4. Extracts the ZIP file’s contents, handling directories and files appropriately:
    ///    - Creates any necessary parent directories for files.
    ///    - Writes extracted files to disk.
    /// 5. Optionally sets file permissions on Unix-based systems based on the ZIP archive’s metadata.
    ///
    /// # Errors:
    /// The function returns a `CoreError` if any of the following occurs:
    /// - A failure to download or read the ZIP file (`CoreError::FileReadError`).
    /// - A mismatch between the provided checksum and the downloaded file (`CoreError::ChecksumError`).
    /// - An error opening the ZIP file (`CoreError::ZipFileOpenError`).
    /// - A failure to create directories or files (`CoreError::CreateDirError`, `CoreError::CreateFileError`).
    /// - A failure during file extraction (`CoreError::ZipFileWriteError`).
    /// - A failure setting file permissions on Unix-based systems (`CoreError::PermissionError`).
    pub fn download_layers(&mut self) -> Result<(), CoreError> {
        for layer in &self.manifest.layers {
            let downloader = Downloader::new(layer.url()).map_err(|_| CoreError::DownloadError)?;

            // Retrieve the zipfile path
            let zipfile_path = downloader.zipfile();

            // Open the file at the given path
            let zipfile = File::open(zipfile_path).map_err(|_| CoreError::FileReadError)?;

            // Verify the checksum
            if let Some(checksum) = layer.checksum() {
                if !self.verify_layer_checksum(&zipfile_path, checksum)? {
                    return Err(CoreError::ChecksumError);
                }
            }

            // Use the opened file for the ZipArchive
            let mut archive = ZipArchive::new(BufReader::new(zipfile))
                .map_err(|_| CoreError::ZipFileOpenError)?;

            // Iterate through the ZIP file's entries
            for i in 0..archive.len() {
                let mut file = archive.by_index(i).map_err(|_| CoreError::DownloadError)?;
                let out_path = &self.staging_dir.path().join(file.name());

                // Handle directories and files differently
                if file.is_dir() {
                    std::fs::create_dir_all(&out_path).map_err(|_| CoreError::CreateDirError)?;
                } else {
                    // Create parent directories if necessary
                    if let Some(parent) = out_path.parent() {
                        std::fs::create_dir_all(parent).map_err(|_| CoreError::CreateDirError)?;
                    }

                    // Write file contents
                    let mut out_file =
                        File::create(&out_path).map_err(|_| CoreError::CreateFileError)?;
                    copy(&mut file, &mut out_file).map_err(|_| CoreError::ZipFileWriteError)?;
                }
            }
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
