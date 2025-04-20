use crate::error::ManifestError;
use config::Config;
use std::fs::File;
use std::path::Path;
use walkdir::WalkDir;

#[derive(Default)]
pub struct Loader {
    settings: Config,
}

impl Loader {
    pub fn from_dir(startdir: &Path) -> Result<Loader, ManifestError> {
        if !startdir.is_dir() {
            return Err(ManifestError::FileOpenError);
        }
        let mut loader = Loader::default();
        let mut settings = Config::builder();
        for entry in WalkDir::new(startdir)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| {
                e.file_type().is_file()
                    && e.path().extension().and_then(|s| s.to_str()) == Some("toml")
            })
        {
            settings = settings.add_source(config::File::from(entry.path()));
        }
        let settings = settings
            .build()
            .map_err(|_| ManifestError::ConfigBuildError)?;
        loader.settings = settings;
        Ok(loader)
    }
}
