mod error;

use config::{Config, File};
use directories::ProjectDirs;
pub use error::ManifestError;
use std::path::Path;
use walkdir::WalkDir;

pub struct Manifest {
    config: Config,
}

impl Manifest {
    pub fn load(base_dir: &Path) -> Result<Self, ManifestError> {
        let mut builder = Config::builder();

        if !base_dir.is_dir() {
            return Err(ManifestError::BaseDirError);
        }

        // 1. Load the main config file from your homedir if present
        if let Some(proj_dirs) = ProjectDirs::from("com", "area536", "doscontainer") {
            let main_config_path = proj_dirs.config_dir().join("config.toml");
            if main_config_path.exists() {
                builder = builder.add_source(File::from(main_config_path));
            }
        }

        // 2. Load all config.toml files found in the hierarchy passed in
        builder = WalkDir::new(base_dir)
            .into_iter()
            .filter_map(Result::ok) // Ignore errors
            .filter(|e| e.file_type().is_file() && e.file_name() == "config.toml")
            .fold(builder, |b, entry| {
                b.add_source(File::from(entry.path().to_path_buf()))
            });

        // 3. Build the final merged configuration
        Ok(Manifest {
            config: builder.build().map_err(ManifestError::ConfigError)?,
        })
    }
}
