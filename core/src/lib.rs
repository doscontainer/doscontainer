mod error;


use disk::Disk;
use filesystem::FileSystem;
use manifest::Manifest;
use operatingsystem::OperatingSystem;
use tempfile::TempDir;

pub struct DosContainer {
    disk: Box<dyn Disk>,
    os: Option<OperatingSystem>,
    manifest: Option<Manifest>,
    fs: Box<dyn FileSystem>,
    staging_dir: Option<TempDir>,
}
