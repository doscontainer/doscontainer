use std::{fmt, str::FromStr};

use serde::Deserialize;

use crate::error::SpecError;

#[derive(Debug)]
pub enum FileSystemType {
    Fat12,
}

impl fmt::Display for FileSystemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemType::Fat12 => Ok(write!(f, "FAT12")?),
        }
    }
}

impl<'de> Deserialize<'de> for FileSystemType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;
        let s: String = Deserialize::deserialize(deserializer)?;
        FileSystemType::from_str(&s).map_err(|e| D::Error::custom(e.to_string()))
    }
}

impl FromStr for FileSystemType {
    type Err = SpecError;

    fn from_str(input: &str) -> Result<Self, SpecError> {
        match input.to_lowercase().as_str() {
            "fat12" | "fat 12" => Ok(FileSystemType::Fat12),
            _ => Err(SpecError::InvalidFileSystemType),
        }
    }
}
