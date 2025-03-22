use config::{Config, File, FileFormat};
use directories::ProjectDirs;
use std::path::{Path, PathBuf};

pub struct Manifest {
    config: Config,
}

impl Manifest {
    pub fn new(manifest_path: &Path) -> Self {
        let proj_dirs = ProjectDirs::from("com", "area536", "doscontainer").unwrap();
        let config_path = proj_dirs.config_dir().join("config.toml");

        let settings = Config::builder()
            .add_source(config::File::with_name(config_path.to_str().unwrap()))
            .build()
            .unwrap();

        return Manifest { config: settings };
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use directories::UserDirs;

    #[cfg(unix)]
    #[test]
    fn configpath() {
        let proj_dirs = ProjectDirs::from("com", "area536", "doscontainer").unwrap();
        let config_path = proj_dirs.config_dir();

        let user_dirs = UserDirs::new().expect("Failed to get user dirs.");
        let home_dir = user_dirs.home_dir();

        assert_eq!(home_dir.join(".config").join("doscontainer"), config_path);
    }
}
