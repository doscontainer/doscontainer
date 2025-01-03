use thiserror::Error;

#[derive(Error, Debug)]
pub enum DownloadError{
    #[error("Error handling disk IO")]
    Io(#[from] std::io::Error),
    #[error("Error handling HTTP communications.")]
    Reqwest(#[from] reqwest::Error),
}