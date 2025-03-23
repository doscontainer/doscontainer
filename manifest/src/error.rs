use config::ConfigError;

pub enum ManifestError {
    BaseDirError,
    ConfigError(ConfigError),
}

impl From<ConfigError> for ManifestError {
    fn from(err: ConfigError) -> Self {
        ManifestError::ConfigError(err)
    }
}
