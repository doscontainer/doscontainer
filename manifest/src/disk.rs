use std::{fmt, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::Geometry;

#[derive(Debug, Deserialize, Serialize)]
pub struct Disk {
    pub name: PathBuf,
    pub label: Option<String>,
    pub size: Option<u32>,
    #[serde(default = "default_filesystem")]
    pub filesystem: String,
    #[serde(default = "default_disktype")]
    pub disktype: String,
    #[serde(default = "default_harddisktype")]
    pub harddisktype: String,
    pub geometry: Option<Geometry>,
}

fn default_disktype() -> String {
    "HardDisk".to_string()
}

fn default_filesystem() -> String {
    "FAT12".to_string()
}
fn default_harddisktype() -> String {
    "CUSTOM".to_string()
}

impl fmt::Display for Disk {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Disk Name        : {:?}\nLabel            : {}\nSize (MB)        : {}\nDisk Type        : {}\n",
            self.name,
            self.label.as_deref().unwrap_or("N/A"),
            self.size.unwrap_or(0),
            self.display_disktype()
        )
    }
}

impl Disk {
    pub fn display_disktype(&self) -> String {
        match self.disktype.to_ascii_lowercase().as_str() {
            "ide" => "Hard disk: modern IDE/ATA.".to_string(),
            "pcxt" => "Hard disk: IBM XT compatible".to_string(),
            "pcat" => "Hard disk: IBM AT compatible".to_string(),
            "f35_720" => "Floppy: 3.5\" DD 720KB".to_string(),
            "f35_1440" => "Floppy: 3.5\" HD 1.44MB".to_string(),
            "f35_2880" => "Floppy: 3.5\" XD 2.88MB".to_string(),
            "f525_160" => "Floppy: 5.25\" 160KB".to_string(),
            "f525_180" => "Floppy: 5.25\" 180KB".to_string(),
            "f525_320" => "Floppy: 5.25\" 320KB".to_string(),
            "f525_360" => "Floppy: 5.25\" 360KB".to_string(),
            "f525_1200" => "Floppy: 5.25\" 1.2MB".to_string(),
            _ => "Unknown disk type. Please report this as a bug!".to_string(),
        }
    }
}