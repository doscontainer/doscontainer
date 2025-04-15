use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use ftp::{FtpError, FtpStream};
use tempfile::tempdir;
use url::Url;

use crate::ManifestError;
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
            Err(_) => Err(ManifestError::InvalidUrl),
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
    /// Downloads and stages the source file for this layer.
    ///
    /// This method is only valid for layers of type [`Software`]. It attempts to download
    /// the file specified in `self.url` using the appropriate protocol handler, based on
    /// the URL scheme:
    ///
    /// - `http` and `https` are handled via [`download_http`].
    /// - `ftp` is handled via [`download_ftp`].
    ///
    /// On successful download, the local path to the downloaded file is stored in `self.zipfile_path`.
    ///
    /// # Errors
    ///
    /// Returns a [`ManifestError`] if:
    ///
    /// - The layer is not of type `Software` (`InvalidLayerType`).
    /// - No URL is present for the layer (`MissingUrl`).
    /// - The URL scheme is unsupported (`UnsupportedUrlScheme`).
    /// - The actual download operation fails, as reported by `download_http` or `download_ftp`.
    pub fn download(&mut self) -> Result<(), ManifestError> {
        if self.layer_type != Software {
            return Err(ManifestError::InvalidLayerType);
        }

        let url = self.url.as_ref().ok_or(ManifestError::MissingUrl)?;

        let zipfile_path = match url.scheme() {
            "http" | "https" => self.download_http()?,
            "ftp" => self.download_ftp()?,
            _ => return Err(ManifestError::UnsupportedUrlScheme),
        };

        self.zipfile_path = Some(zipfile_path);
        Ok(())
    }

    /// Extract the zipfile into a staging directory, ready for further processing.
    pub fn stage(&mut self) -> Result<(), ManifestError> {
        if self.layer_type != Software {
            return Err(ManifestError::InvalidLayerType);
        }
        if let Some(_zipfile) = &self.zipfile_path {
            let staging_path = tempdir().map_err(|_| ManifestError::TempDirError)?;
            self.staging_path = Some(staging_path.into_path());
            return Ok(());
        }
        Err(ManifestError::TempDirError)
    }

    /// Downloads the file from the Layer's HTTP(S) URL into a temporary directory.
    ///
    /// This method attempts to download the file specified by `self.url` over HTTP or HTTPS.
    /// The downloaded file is saved in a newly created temporary directory, and its path
    /// is returned on success.
    ///
    /// # Errors
    ///
    /// Returns a [`ManifestError`] if:
    ///
    /// - `self.url` is not set (`InvalidUrl`)
    /// - The URL does not contain a valid file name (`InvalidUrl`)
    /// - The temporary directory cannot be created (`TempDirError`)
    /// - The HTTP request fails to send (`HttpRequestError`)
    /// - The HTTP response indicates a non-success status (`HttpRequestError`)
    /// - The response body cannot be read (`HttpRequestError`)
    /// - The file cannot be created locally (`DownloadError`)
    /// - The response body cannot be written to disk (`DownloadError`)
    ///
    /// # Returns
    ///
    /// On success, returns the full path to the downloaded file within the temporary directory.
    #[allow(clippy::manual_next_back)]
    fn download_http(&mut self) -> Result<PathBuf, ManifestError> {
        let download_path = tempdir().map_err(|_| ManifestError::TempDirError)?;

        let url = self.url.as_ref().ok_or(ManifestError::InvalidUrl)?;

        let file_name = url
            .path_segments()
            .and_then(|segments| segments.last())
            .filter(|name| !name.is_empty())
            .ok_or(ManifestError::InvalidUrl)?;

        let target_path = download_path.path().join(file_name);

        let response = attohttpc::get(url)
            .send()
            .map_err(|_| ManifestError::HttpRequestError)?;

        if !response.is_success() {
            return Err(ManifestError::HttpRequestError);
        }

        let content = response
            .bytes()
            .map_err(|_| ManifestError::HttpRequestError)?;

        let mut file = File::create(&target_path).map_err(|_| ManifestError::DownloadError)?;

        file.write_all(&content)
            .map_err(|_| ManifestError::DownloadError)?;

        Ok(target_path)
    }

    /// Downloads a file from an FTP server to a temporary directory.
    ///
    /// This method connects to the FTP server specified in `self.url`, authenticates using either the
    /// credentials provided in the URL or anonymous login if none are present, and retrieves the file
    /// located at the URL's path. The file is saved in a newly created temporary directory.
    ///
    /// The FTP transfer is performed in binary mode to preserve file integrity.
    ///
    /// # Returns
    ///
    /// On success, returns a [`PathBuf`] pointing to the downloaded file within the temporary directory.
    ///
    /// # Errors
    ///
    /// Returns a [`ManifestError`] if:
    ///
    /// - The URL is missing, invalid, or lacks necessary components such as a host or file name.
    /// - The temporary directory could not be created.
    /// - The FTP connection could not be established.
    /// - Authentication with the FTP server failed.
    /// - The transfer type could not be set to binary mode.
    /// - The file could not be retrieved or written locally.
    /// - The FTP connection could not be closed gracefully.
    ///
    /// # FTP Credentials
    ///
    /// - If the URL includes a username and/or password, those are used for authentication.
    /// - If no credentials are provided, it defaults to:
    ///   - Username: `"anonymous"`
    ///   - Password: `"doscontainer@area536.com"`
    ///
    /// # Notes
    ///
    /// - The caller is responsible for managing the lifecycle of the temporary directory.
    /// - The temporary directory is created using the system's default temporary directory mechanism.
    ///
    /// # See Also
    ///
    /// [`tempdir`](https://docs.rs/tempfile/latest/tempfile/fn.tempdir.html)
    /// [`FtpStream`](https://docs.rs/ftp/latest/ftp/struct.FtpStream.html)
    fn download_ftp(&mut self) -> Result<PathBuf, ManifestError> {
        let download_path = tempdir().map_err(|_| ManifestError::TempDirError)?;
        let url = self.url.as_ref().ok_or(ManifestError::InvalidUrl)?;

        let hostname = url.host_str().ok_or(ManifestError::InvalidUrl)?;
        let port = url.port_or_known_default().unwrap_or(21);

        let path = url.path();
        if path.is_empty() {
            return Err(ManifestError::InvalidUrl);
        }

        let file_name = path
            .split('/')
            .next_back()
            .filter(|s| !s.is_empty())
            .ok_or(ManifestError::InvalidUrl)?;
        let file_path = download_path.path().join(file_name);

        let mut ftp =
            FtpStream::connect((hostname, port)).map_err(|_| ManifestError::FtpConnectionError)?;

        let username = if url.username().is_empty() {
            "anonymous"
        } else {
            url.username()
        };
        let password = url.password().unwrap_or("doscontainer@area536.com");

        ftp.login(username, password)
            .map_err(|_| ManifestError::FtpAuthenticationError)?;

        ftp.transfer_type(ftp::types::FileType::Binary)
            .map_err(|_| ManifestError::FtpTransferTypeError)?;

        ftp.retr(path, |stream| {
            let mut local_file = File::create(&file_path).map_err(FtpError::ConnectionError)?;
            let mut buffer = [0u8; 8192];

            loop {
                let bytes_read = stream
                    .read(&mut buffer)
                    .map_err(FtpError::ConnectionError)?;
                if bytes_read == 0 {
                    break;
                }
                local_file
                    .write_all(&buffer[..bytes_read])
                    .map_err(FtpError::ConnectionError)?;
            }
            Ok(())
        })
        .map_err(|_| ManifestError::FtpConnectionError)?;

        ftp.quit().map_err(|_| ManifestError::FtpConnectionError)?;

        Ok(file_path)
    }
}

impl Default for Layer {
    fn default() -> Layer {
        Layer {
            layer_type: Software,
            url: None,
            zipfile_path: None,
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
