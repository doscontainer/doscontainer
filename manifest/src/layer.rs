use ftp::{FtpError, FtpStream};
use log::info;
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::io::{BufReader, Seek, Write};
use std::{fmt, fs};
use std::{fs::File, io::Read};
use tempfile::{tempdir, NamedTempFile, TempDir};
use url::Url;
use zip::ZipArchive;

use crate::error::ManifestError;
use crate::os::{OsVendor, OsVersion};

#[derive(Default, Deserialize)]
pub struct Layer {
    url: Option<Url>,
    checksum: Option<String>,
    min_dos: Option<OsVersion>,
    max_dos: Option<OsVersion>,
    dos_vendor: Option<OsVendor>,
    #[serde(skip_deserializing)]
    zipfile_path: Option<NamedTempFile>,
    #[serde(skip_deserializing)]
    staging_path: Option<TempDir>,
}

impl Layer {
    pub fn set_url(&mut self, url: &str) -> Result<(), ManifestError> {
        match Url::parse(url) {
            Ok(_) => {
                self.url = Some(Url::parse(url).unwrap());
                Ok(())
            }
            Err(_) => Err(ManifestError::InvalidUrl),
        }
    }

    pub fn url(&self) -> &Option<Url> {
        &self.url
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
        let url = self.url.as_ref().ok_or(ManifestError::MissingUrl)?;

        let zipfile_path = match url.scheme() {
            "http" | "https" => self.download_http()?,
            "ftp" => self.download_ftp()?,
            _ => return Err(ManifestError::UnsupportedUrlScheme),
        };

        if let Some(checksum) = &self.checksum {
            let file =
                File::open(&zipfile_path).map_err(|_| ManifestError::ChecksumVerificationFailed)?;

            let mut reader = BufReader::new(file);
            let mut hasher = Sha256::new();
            let mut buffer = [0u8; 8192];

            loop {
                let n = reader
                    .read(&mut buffer)
                    .map_err(|_| ManifestError::ChecksumVerificationFailed)?;
                if n == 0 {
                    break;
                }
                hasher.update(&buffer[..n]);
            }

            let calculated = hasher.finalize();
            let calculated_hex = format!("{:x}", calculated);

            if calculated_hex != checksum.to_lowercase() {
                return Err(ManifestError::ChecksumVerificationFailed);
            }
        }

        self.zipfile_path = Some(zipfile_path);
        self.stage()?;
        Ok(())
    }

    fn stage(&mut self) -> Result<(), ManifestError> {
        let zipfile = self
            .zipfile_path
            .as_ref()
            .ok_or(ManifestError::TempDirError)?;
        let staging_path = tempdir().map_err(|_| ManifestError::TempDirError)?;
        let mut archive = ZipArchive::new(zipfile).map_err(|_| ManifestError::ZipFileCorrupt)?;
        let zipfile_logdisplay = zipfile.path();
        info!(target: "dosk8s_events", "Start extracting archive {zipfile_logdisplay:?}.");

        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|_| ManifestError::ZipFileCorrupt)?;
            let target = staging_path.path().join(file.name());

            if file.is_dir() {
                fs::create_dir_all(&target).map_err(|_| ManifestError::FileOpenError)?;
            } else {
                if let Some(parent) = target.parent() {
                    fs::create_dir_all(parent).map_err(|_| ManifestError::FileOpenError)?;
                }

                let mut outfile =
                    fs::File::create(&target).map_err(|_| ManifestError::FileOpenError)?;
                std::io::copy(&mut file, &mut outfile).map_err(|_| ManifestError::FileOpenError)?;
            }
        }

        info!(target: "dosk8s_events", "Finished extracting archive {zipfile_logdisplay:?}.");
        self.staging_path = Some(staging_path);

        Ok(())
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
    fn download_http(&mut self) -> Result<NamedTempFile, ManifestError> {
        let url = self.url.as_ref().ok_or(ManifestError::InvalidUrl)?;
        info!(target: "dosk8s_events", "Starting HTTP(S) download for {url}.");

        let response = attohttpc::get(url)
            .send()
            .map_err(|_| ManifestError::HttpRequestError)?;

        if !response.is_success() {
            return Err(ManifestError::HttpRequestError);
        }

        let content = response
            .bytes()
            .map_err(|_| ManifestError::HttpRequestError)?;

        let mut tempfile = NamedTempFile::new().map_err(|_| ManifestError::TempDirError)?;

        tempfile
            .write_all(&content)
            .map_err(|_| ManifestError::DownloadError)?;
        info!(target: "dosk8s_events", "Finished HTTP(S) download for {url}.");
        Ok(tempfile)
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
    fn download_ftp(&mut self) -> Result<NamedTempFile, ManifestError> {
        let url = self.url.as_ref().ok_or(ManifestError::InvalidUrl)?;
        info!(target: "dosk8s_events", "Start FTP download from {url}.");
        let hostname = url.host_str().ok_or(ManifestError::InvalidUrl)?;
        let port = url.port_or_known_default().unwrap_or(21);

        let path = url.path();
        if path.is_empty() {
            return Err(ManifestError::InvalidUrl);
        }

        let tempfile = NamedTempFile::new().map_err(|_| ManifestError::TempDirError)?;

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
            let mut local_file = File::create(&tempfile).map_err(FtpError::ConnectionError)?;
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
        info!(target: "dosk8s_events", "Finish FTP download from {url}.");
        Ok(tempfile)
    }

    /// Validate the Layer's own zipfile
    pub fn validate_zip_file(&self) -> Result<(), ManifestError> {
        if let Some(file) = &self.zipfile_path {
            info!(target: "dosk8s_events", "Start validating ZIP file {file:?}");
            let zipfile = File::open(file).map_err(|_| ManifestError::FileOpenError)?;
            let reader = BufReader::new(zipfile);
            self.validate_zip_stream(reader)?;
        } else {
            info!(target: "dosk8s_events", "ZIP file validation failed.");
            return Err(ManifestError::ZipFileNotSet);
        }
        info!(target: "dosk8s_events", "Finish validating ZIP file.");
        Ok(())
    }

    /// Generalized implementation so that validation is properly testable
    fn validate_zip_stream<R: Read + Seek>(&self, reader: R) -> Result<(), ManifestError> {
        // ..when they have an actual zipfile set.
        let mut archive = ZipArchive::new(reader).map_err(|_| ManifestError::FileOpenError)?;

        // Loop over all files in the archive
        for i in 0..archive.len() {
            let mut file = archive
                .by_index(i)
                .map_err(|_| ManifestError::ZipFileCorrupt)?;

            // We can't CRC-check a directory
            if file.is_dir() {
                continue;
            }

            let expected_crc = file.crc32();
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)
                .map_err(|_| ManifestError::ZipFileCorrupt)?;

            // Do the actual CRC check
            let actual_crc = crc32fast::hash(&buffer);
            if expected_crc != actual_crc {
                return Err(ManifestError::ZipFileCorrupt);
            }
        }
        Ok(())
    }
}

impl fmt::Display for Layer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Layer")?;
        if let Some(url) = &self.url {
            writeln!(f, "  URL         : {}", url)?;
        }
        if let Some(checksum) = &self.checksum {
            writeln!(f, "  Checksum    : {}", checksum)?;
        }
        if let Some(min_dos) = &self.min_dos {
            writeln!(f, "  Minimum DOS version: {}", min_dos)?;
        }
        if let Some(max_dos) = &self.max_dos {
            writeln!(f, "  Maximum DOS version: {}", max_dos)?;
        }
        Ok(())
    }
}
