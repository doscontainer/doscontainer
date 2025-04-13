use crate::ManifestError;
use attohttpc::get;
use ftp::{FtpError, FtpStream};
use std::fs::File;
use std::io::Write;
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
    zipfile_path: Option<PathBuf>,
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
                let zipfile_path = match download_url.scheme() {
                    "http" | "https" => self.download_http()?,
                    "ftp" => self.download_ftp()?,
                    // We have a URL, but we don't support the scheme type (yet).
                    _ => return Err(ManifestError::UnsupportedUrlScheme),
                };
                self.zipfile_path = Some(zipfile_path);
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
        let download_path = tempdir().map_err(|_| ManifestError::TempDirError)?;

        if let Some(url) = &self.url {
            // Extract the file name from the URL's path.
            let path = url.path();
            let file_name = path.split('/').last().ok_or(ManifestError::InvalidUrl)?;
            if file_name.is_empty() {
                return Err(ManifestError::InvalidUrl);
            }
            // Compose the full path for the ZIP file
            let zipfile_path = download_path.path().join(file_name);

            // Perform the HTTP download
            let response = attohttpc::get(url)
                .send()
                .map_err(|_| ManifestError::HttpRequestError)?;

            if !response.is_success() {
                return Err(ManifestError::HttpRequestError);
            }

            let mut file = File::create(&zipfile_path).map_err(|_| ManifestError::DownloadError)?;
            // Write the response body to the file.
            let content = response
                .bytes()
                .map_err(|_| ManifestError::HttpRequestError)?;
            file.write_all(&content)
                .map_err(|_| ManifestError::DownloadError)?;

            return Ok(zipfile_path);
        }
        Err(ManifestError::InvalidUrl)
    }

    /// Download the Layer's url over FTP
    fn download_ftp(&mut self) -> Result<PathBuf, ManifestError> {
        let download_path = tempdir().map_err(|_| ManifestError::TempDirError)?;
        if let Some(url) = &self.url {
            // Extract the file name from the URL's path
            let path = url.path();
            let file_name = path.split('/').last().ok_or(ManifestError::InvalidUrl)?;
            if file_name.is_empty() {
                return Err(ManifestError::InvalidUrl);
            }

            // Compose the full path for the ZIP file
            let zipfile_path = download_path.path().join(file_name);

            // Perform the FTP download
            let hostname = url.host_str().ok_or(ManifestError::InvalidUrl)?;
            let port = url.port_or_known_default().unwrap_or(21);
            let path = url.path();
            if path.is_empty() {
                return Err(ManifestError::InvalidUrl);
            }

            // Extract the file name from the URL's path.
            let file_name = path.split('/').last().ok_or(ManifestError::InvalidUrl)?;
            if file_name.is_empty() {
                return Err(ManifestError::InvalidUrl);
            }

            let file_path = download_path.path().join(file_name);
            // Perform the FTP transaction.
            let mut ftp = FtpStream::connect((hostname, port))
                .map_err(|_| ManifestError::FtpConnectionError)?;

            // Authenticate with anonymous credentials if no username/password is provided.
            let username = if url.username().is_empty() {
                "anonymous"
            } else {
                url.username()
            };
            let password = url.password().unwrap_or("doscontainer@area536.com");
            ftp.login(username, password)
                .map_err(|_| ManifestError::FtpAuthenticationError)?;

            // Switch to binary mode for file transfers.
            ftp.transfer_type(ftp::types::FileType::Binary)
                .map_err(|_| ManifestError::FtpTransferTypeError)?;

            // Start retrieving the file.
            ftp.retr(path, |stream| {
                let mut local_file =
                    File::create(&file_path).map_err(|e| FtpError::ConnectionError(e))?;
                let mut buffer = [0u8; 8192];
                loop {
                    let bytes_read = stream
                        .read(&mut buffer)
                        .map_err(|e| FtpError::ConnectionError(e))?;
                    if bytes_read == 0 {
                        break; // EOF
                    }
                    local_file
                        .write_all(&buffer[..bytes_read])
                        .map_err(|e| FtpError::ConnectionError(e))?;
                }
                Ok(())
            })
            .map_err(|_| ManifestError::FtpConnectionError)?;

            // Close the FTP connection gracefully.
            ftp.quit().map_err(|_| ManifestError::FtpConnectionError)?;

            // Return the path to the downloaded file.
            return Ok(zipfile_path);
        }
        Err(ManifestError::InvalidUrl)
    }
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            layer_type: Software,
            url: None,
            zipfile_path: None,
            disk_category: None,
            disk_type: None,
            filesystem: None,
            cylinders: None,
            heads: None,
            sectors: None,
        }
    }
}
