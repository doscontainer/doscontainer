use std::{fmt, str::FromStr};

use crate::error::FileSystemError;

#[derive(Debug, PartialEq)]
pub struct EntryName {
    pub filename: String,
    pub extension: String,
}

impl FromStr for EntryName {
    type Err = FileSystemError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "." || s == ".." {
            return Err(FileSystemError::CannotCreateDotfiles);
        }

        let parts: Vec<&str> = s.split('.').collect();

        // Must have at least one part for a valid file name
        if parts.len() == 0 {
            return Err(FileSystemError::EmptyFileName);
        }

        // Can't have more than two parts
        if parts.len() > 2 {
            return Err(FileSystemError::TooManyFileNameParts);
        }

        let name_part = parts[0].trim().to_ascii_uppercase();
        if name_part.is_empty() {
            return Err(FileSystemError::EmptyFileName);
        }

        if name_part.len() > 8 {
            return Err(FileSystemError::FileNameTooLong);
        }

        let ext_part = if parts.len() == 2 {
            parts[1].trim().to_ascii_uppercase()
        } else {
            String::new()
        };
        if ext_part.len() > 3 {
            return Err(FileSystemError::ExtensionTooLong);
        }

        // Check each character is valid and uppercase
        if !name_part.chars().all(|c| Self::is_valid_char(c)) {
            return Err(FileSystemError::InvalidCharInName);
        }

        if !ext_part.chars().all(|c| Self::is_valid_char(c)) {
            return Err(FileSystemError::InvalidCharInExt);
        }

        // We have a valid name struct
        Ok(Self {
            filename: name_part,
            extension: ext_part,
        })
    }
}

impl EntryName {
    pub fn to_string(&self) -> String {
        if self.extension.is_empty() {
            self.filename.clone()
        } else {
            format!("{}.{}", self.filename, self.extension)
        }
    }
    pub fn is_valid_char(c: char) -> bool {
        matches!(c,
            'A'..='Z' | '0'..='9' |
            '\u{0020}' | '!' | '#' | '$' | '%' | '&' | '\'' |
            '(' | ')' | '-' | '@' | '^' | '_' | '`' | '{' | '}' | '~'
        )
    }
}

impl fmt::Display for EntryName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.extension.is_empty() {
            write!(f, "{}", self.filename)
        } else {
            write!(f, "{}.{}", self.filename, self.extension)
        }
    }
}
