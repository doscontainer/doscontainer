use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
    time::Duration,
};

use error::DownloadError;
use reqwest::{blocking::Client, header::RANGE};
use tempfile::TempDir;

mod error;

pub struct Downloader {
    zipfile: PathBuf,
    zipdir: TempDir,
}

impl Downloader {
    const MAX_RETRIES: u32 = 3;
    const TIMEOUT: Duration = Duration::from_secs(30);
    const BUFFER_SIZE: usize = 8192;

    pub fn new() -> Result<Self, DownloadError> {
        let zipdir = TempDir::new()?;
        let zipfile = zipdir.path().join("downloaded.zip");

        Ok(Downloader { zipfile, zipdir })
    }

    fn download_zip(url: &str, tempdir: &TempDir) -> Result<PathBuf, DownloadError> {
        let client = Client::builder().timeout(Self::TIMEOUT).build()?;
        let zipfile = tempdir.path().join("downloaded.zip");

        let mut attempts = 0;
        loop {
            match Self::attempt_download(&client, url, &zipfile) {
                Ok(_) => break Ok(zipfile),
                Err(e) if attempts < Self::MAX_RETRIES => attempts += 1,
                Err(e) => break Err(e),
            }
        }
    }

    fn attempt_download(
        client: &Client,
        url: &str,
        zipfile: &PathBuf,
    ) -> Result<(), DownloadError> {
        let mut file = if zipfile.exists() {
            OpenOptions::new().write(true).open(&zipfile)?
        } else {
            File::create(&zipfile)?
        };

        let downloaded_size = file.metadata()?.len();
        let mut response = client
            .get(url)
            .header(RANGE, format!("bytes={}-", downloaded_size))
            .send()?
            .error_for_status()?;

        let mut buffer = vec![0u8; Self::BUFFER_SIZE];
        while let Ok(n) = response.read(&mut buffer) {
            if n == 0 {
                break;
            }
            file.write_all(&buffer[..n])?;
        }

        Ok(())
    }
}
