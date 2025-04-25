mod error;

use std::{
    fs::File,
    io::{copy, BufReader, Read},
    path::Path,
};

use disk::{disktype::DiskType, floppy::Floppy, Disk};
use error::CoreError;
use filesystem::{fat12::Fat12, FileSystem};
use manifest::Manifest;
use operatingsystem::OperatingSystem;
use sha2::{Digest, Sha256};
use tempfile::TempDir;
use zip::ZipArchive;

pub struct DosContainer {
    disk: Box<dyn Disk>,
    os: Option<OperatingSystem>,
    manifest: Option<Manifest>,
    fs: Box<dyn FileSystem>,
    staging_dir: Option<TempDir>,
}
