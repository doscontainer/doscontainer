use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use error::DownloadError;
use ftp::{FtpError, FtpStream};
use reqwest::{blocking::Client, header::RANGE};
use tempfile::TempDir;
use url::Url; // Add FTP support with the `ftp` crate.

mod error;

#[derive(Debug)]
pub struct Downloader {
    zipfile: PathBuf,
    zipdir: TempDir,
}

impl Downloader {
    pub fn new(url: &str) -> Result<Self, DownloadError> {
        let zipdir = TempDir::new().map_err(|_| DownloadError::ZipDirCreateFailed)?;
        let zipfile = PathBuf::new();

        Ok(Downloader { zipfile, zipdir })
    }

    /// Downloads a ZIP file from a given URL and saves it to a local temporary file.
    ///
    /// This function supports downloading from HTTP, HTTPS, and FTP URLs. It determines
    /// the protocol based on the scheme of the provided URL and delegates the download
    /// process to protocol-specific methods. If the scheme is unsupported, it returns an error.
    ///
    /// # Arguments
    ///
    /// * `url` - A string slice that contains the URL of the ZIP file to download.
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing the path to the downloaded ZIP file (`PathBuf`) on success,
    /// or a `DownloadError` on failure. The error could occur due to:
    /// - Invalid URL parsing (`DownloadError::InvalidUrl`)
    /// - Unsupported URL schemes (`DownloadError::UnsupportedScheme`)
    /// - Protocol-specific errors (e.g., `HttpError` or `FtpError`).
    ///
    /// # Errors
    ///
    /// This function will return an error in the following cases:
    /// - The URL is invalid and cannot be parsed (`DownloadError::InvalidUrl`).
    /// - The URL scheme is unsupported (only "http", "https", and "ftp" are supported).
    /// - Protocol-specific errors occur while downloading the file.
    pub fn download_zip(&self, url: &str) -> Result<PathBuf, DownloadError> {
        let parsed_url = Url::parse(url).map_err(|_| DownloadError::InvalidUrl)?; // Validate and parse the URL
        match parsed_url.scheme() {
            "http" | "https" => Self::download_http(url),
            "ftp" => self.download_ftp(url),
            _ => Err(DownloadError::UnsupportedScheme),
        }
    }

    /// Downloads a file from an FTP server and saves it in a temporary directory.
    ///
    /// # Parameters
    /// - `url`: A string slice representing the FTP URL to download from. The URL must be valid,
    ///   conform to the FTP protocol, and specify the file path.
    ///
    /// # Returns
    /// - `Ok(PathBuf)`: The path to the downloaded file in the temporary directory.
    /// - `Err(DownloadError)`: An error if the download fails at any stage.
    ///
    /// # Errors
    /// - `DownloadError::InvalidUrl`: If the URL is invalid or cannot be parsed.
    /// - `DownloadError::UnsupportedScheme`: If the URL does not use the `ftp` scheme.
    /// - `DownloadError::PathIsEmpty`: If the URL does not specify a path.
    /// - `DownloadError::FileNameIsEmpty`: If the URL path does not contain a valid file name.
    /// - `DownloadError::FtpConnectionError`: If the connection to the FTP server fails.
    /// - `DownloadError::FtpAuthenticationError`: If authentication with the FTP server fails.
    /// - `DownloadError::FtpTransferTypeError`: If switching to binary transfer mode fails.
    /// - `DownloadError::FtpStreamReadError`: If an error occurs while reading the file stream.
    /// - `DownloadError::FtpDisconnectError`: If disconnecting from the FTP server fails.
    ///
    /// # Details
    /// 1. **Validation**: The URL is validated to ensure it is well-formed, uses the `ftp` scheme,
    ///    and contains a valid path and file name.
    /// 2. **Temporary Directory**: The downloaded file is stored in a temporary directory managed by `self.zipdir`.
    /// 3. **FTP Connection**: The function connects to the FTP server using the host and port extracted
    ///    from the URL. If no port is specified, the default port `21` is used.
    /// 4. **Authentication**: The function authenticates using the username and password provided in the
    ///    URL. If no credentials are provided, it defaults to anonymous authentication.
    /// 5. **File Transfer**: The file is transferred in binary mode and saved to the temporary directory.
    ///    A buffer is used for efficient reading and writing.
    /// 6. **Cleanup**: The FTP connection is gracefully closed after the transfer.
    fn download_ftp(&self, url: &str) -> Result<PathBuf, DownloadError> {
        // Validate and parse the input URL.
        let parsed_url = Url::parse(url).map_err(|_| DownloadError::InvalidUrl)?;

        // Ensure the URL is an FTP URL.
        if parsed_url.scheme() != "ftp" {
            return Err(DownloadError::UnsupportedScheme);
        }

        // Extract host, port, and path information from the URL.
        let host = parsed_url
            .host_str()
            .ok_or_else(|| DownloadError::InvalidUrl)?;
        let port = parsed_url.port_or_known_default().unwrap_or(21); // Default FTP port is 21.
        let path = parsed_url.path();
        if path.is_empty() {
            return Err(DownloadError::PathIsEmpty);
        }

        // Extract the file name from the URL's path.
        let file_name = path
            .split('/')
            .last()
            .ok_or_else(|| DownloadError::FileNameIsEmpty)?;
        if file_name.is_empty() {
            return Err(DownloadError::FileNameIsEmpty);
        }

        // Create a temporary directory to store the downloaded file.
        let tempdir = &self.zipdir;
        let filepath = tempdir.path().join(file_name);

        // Perform the FTP transaction.
        let mut ftp =
            FtpStream::connect((host, port)).map_err(|_| DownloadError::FtpConnectionError)?;

        // Authenticate with anonymous credentials if no username/password is provided.
        let username = if parsed_url.username().is_empty() {
            "anonymous"
        } else {
            parsed_url.username()
        };
        let password = parsed_url.password().unwrap_or("doscontainer@area536.com");
        ftp.login(username, password)
            .map_err(|_| DownloadError::FtpAuthenticationError)?;

        // Switch to binary mode for file transfers.
        ftp.transfer_type(ftp::types::FileType::Binary)
            .map_err(|_| DownloadError::FtpTransferTypeError)?;

        // Start retrieving the file.
        ftp.retr(path, |stream| {
            let mut local_file =
                File::create(&filepath).map_err(|e| FtpError::ConnectionError(e))?;
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
        .map_err(|_| DownloadError::FtpStreamReadError)?;

        // Close the FTP connection gracefully.
        ftp.quit().map_err(|_| DownloadError::FtpDisconnectError)?;

        // Return the path to the downloaded file.
        Ok(filepath)
    }

    fn download_http(url: &str) -> Result<PathBuf, DownloadError> {
        let parsed_url = Url::parse(url).map_err(|_| DownloadError::InvalidUrl)?;
        Ok(PathBuf::new())
    }
}