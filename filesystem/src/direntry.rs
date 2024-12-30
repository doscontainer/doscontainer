use chrono::{DateTime, Local};
use operatingsystem::OperatingSystem;
use uuid::Uuid;

use crate::error::FileSystemError;

pub struct DirEntry {
    entry_type: DirEntryType,
    extension: Option<String>,
    file_size: usize,
    last_modified_time: DateTime<Local>,
    name: Option<String>,
    parent: Option<Uuid>,
    allocated_clusters: Vec<usize>,
    uuid: Uuid,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum DirEntryType {
    EmptyPlaceholder,
    File,
    Directory,
    SysFile,
    VolumeLabel,
}

impl DirEntry {
    /// Serializes the `DirEntry` into a sequence of bytes that the FAT filesystem uses to
    /// populate the on-disk directory structures for OS'es that support them.
    ///
    /// This method determines the type of directory entry (e.g., file, directory, system file,
    /// volume label, or placeholder) and delegates the serialization process to private helper
    /// methods. Each helper method is responsible for producing the correct on-disk representation
    /// of the entry according to the FAT file system's specification.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: A vector of bytes representing the serialized `DirEntry`.
    /// - `Err(FileSystemError)`: If serialization fails due to an invalid entry type or an
    ///   internal error in one of the helper methods.
    ///
    /// # Variants
    /// The serialization behavior is determined by the value of `self.entry_type`:
    /// - `DirEntryType::EmptyPlaceholder`: Represents an unused directory entry and
    ///   produces a placeholder byte sequence.
    /// - `DirEntryType::Directory`: Represents a subdirectory entry and is serialized
    ///   using the directory-specific method.
    /// - `DirEntryType::File`: Represents a file entry and is serialized accordingly.
    /// - `DirEntryType::SysFile`: Represents a system file entry (if applicable).
    /// - `DirEntryType::VolumeLabel`: Represents a volume label entry.
    ///
    /// # Errors
    /// - If the `DirEntryType` is invalid or unsupported, or if any of the private helper methods
    ///   encounter an error, this function returns a `FileSystemError`.
    ///
    /// # Notes
    /// - This method ensures the serialized bytes conform to the FAT specification.
    /// - Ensure that `self.entry_type` is correctly set before calling this method to
    ///   avoid errors.
    pub fn serialize(&self, os: &OperatingSystem) -> Result<Vec<u8>, FileSystemError> {
        match self.entry_type {
            DirEntryType::EmptyPlaceholder => Ok(Self::serialize_placeholder(os)),
            DirEntryType::Directory => Ok(self.serialize_directory(os)?),
            DirEntryType::File => Ok(self.serialize_file(os)),
            DirEntryType::SysFile => self.serialize_sysfile(),
            DirEntryType::VolumeLabel => self.serialize_volume_label(),
        }
    }

    /// Serializes this directory entry into on-disk bytes for the FAT file system.
    ///
    /// This function produces a 32-byte FAT-compatible representation of the directory entry
    /// based on the operating system. For unsupported operating systems (e.g., DOS 1.xx),
    /// an error is returned.
    ///
    /// # Parameters
    /// - `os`: The operating system variant (`OperatingSystem`) that determines serialization behavior.
    ///
    /// # Returns
    /// - `Ok(Vec<u8>)`: A vector of bytes representing the serialized directory entry.
    /// - `Err(FileSystemError::UnsupportedOperatingSystem)`: If the operating system does not support directories.
    ///
    /// # Notes
    /// - `OperatingSystem::IBMDOS100` and `OperatingSystem::IBMDOS110` do not support directories.
    /// - Currently, only `OperatingSystem::IBMDOS200` (PC-DOS 2.00) is supported for directory serialization.
    /// - Future operating system variants can be added as needed.
    ///
    /// # Errors
    /// - Returns `FileSystemError::UnsupportedOperatingSystem` for operating systems that do not support directories.
    fn serialize_directory(&self, os: &OperatingSystem) -> Result<Vec<u8>, FileSystemError> {
        match os {
            // DOS 1.xx does not support directories at all, that's an error
            OperatingSystem::IBMDOS100 | OperatingSystem::IBMDOS110 => {
                Err(FileSystemError::UnsupportedOperatingSystem)
            }
            // Handle PC-DOS 2.00
            OperatingSystem::IBMDOS200 => {
                let mut output = Vec::new();
                output.extend(self.name_as_bytes());
                output.extend(self.ext_as_bytes());
                output.push(self.type_as_byte());
                output.extend(std::iter::repeat(0).take(10));
                output.extend(self.startcluster_as_bytes()); // 2 bytes for start cluster
                output.extend(std::iter::repeat(0).take(8)); // Filesize is zero
                Ok(output)
            }
            // We don't have a generic case for "all others" just yet, bomb out with an error.
            _ => Err(FileSystemError::UnsupportedOperatingSystem),
        }
    }

    /// Serializes this file entry into a FAT-compatible on-disk format.
    ///
    /// This function generates a 32-byte FAT directory entry based on the given operating system.
    /// It handles variations in how dates, times, and reserved bytes are serialized.
    ///
    /// # Parameters
    /// - `os`: The operating system variant (`OperatingSystem`) that determines serialization behavior.
    ///
    /// # Returns
    /// - A `Vec<u8>` containing the serialized file entry in FAT format.
    ///
    /// # Notes
    /// - `OperatingSystem::IBMDOS100`: Records only the date, with no time field.
    /// - `OperatingSystem::IBMDOS110` and `OperatingSystem::IBMDOS200`: Record both date and time.
    /// - Other operating systems use a default serialization format.
    ///
    /// # Errors
    /// - If `self.last_modified()` returns `None`, default values are used for date and time.
    fn serialize_file(&self, os: &OperatingSystem) -> Vec<u8> {
        let mut output = Vec::with_capacity(32); // Preallocate 32 bytes for efficiency.

        // Add common fields (name, extension, type).
        output.extend(self.name_as_bytes()); // 8 bytes for name.
        output.extend(self.ext_as_bytes()); // 3 bytes for extension.
        output.push(self.type_as_byte()); // 1 byte for type.

        // Match logic based on the operating system.
        match os {
            // IBM PC-DOS 1.10 and 2.00: Record date and time.
            OperatingSystem::IBMDOS110 | OperatingSystem::IBMDOS200 => {
                if let Some((fat_date, fat_time)) = self.last_modified() {
                    output.extend(std::iter::repeat(0).take(10)); // Reserved 10 bytes.
                    output.extend(fat_time.to_le_bytes()); // 2 bytes for time.
                    output.extend(fat_date.to_le_bytes()); // 2 bytes for date.
                } else {
                    output.extend(std::iter::repeat(0).take(14)); // Fallback for missing date/time.
                }
            }

            // IBM PC-DOS 1.00: Only record the date (no time field).
            OperatingSystem::IBMDOS100 => {
                if let Some((fat_date, _)) = self.last_modified() {
                    output.extend(std::iter::repeat(0).take(12)); // Reserved 12 bytes.
                    output.extend(fat_date.to_le_bytes()); // 2 bytes for date.
                } else {
                    output.extend(std::iter::repeat(0).take(14)); // Fallback for missing date.
                }
            }

            // Other operating systems: Use default serialization.
            _ => {
                if let Some((fat_date, fat_time)) = self.last_modified() {
                    output.extend(std::iter::repeat(0).take(10)); // Reserved 10 bytes.
                    output.extend(fat_time.to_le_bytes()); // 2 bytes for time.
                    output.extend(fat_date.to_le_bytes()); // 2 bytes for date.
                } else {
                    output.extend(std::iter::repeat(0).take(14)); // Fallback for missing date/time.
                }
            }
        }

        // Add start cluster and file size.
        output.extend(self.startcluster_as_bytes()); // 2 bytes for start cluster.
        output.extend(&(self.file_size as u32).to_le_bytes()); // 4 bytes for file size.

        output
    }

    /// Serializes a placeholder directory entry for a given operating system.
    ///
    /// This function generates a 32-byte FAT-compatible placeholder entry. The first byte is
    /// a marker, which varies based on the operating system, followed by filler bytes.
    ///
    /// # Parameters
    /// - `os`: The operating system variant (`OperatingSystem`) that determines the marker and filler.
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the serialized placeholder entry.
    ///
    /// # Notes
    /// - `OperatingSystem::IBMDOS100`: Uses `0xE5` as the deleted file marker and `0xF6` as filler.
    /// - `OperatingSystem::IBMDOS110`: Uses `0xE5` as the deleted file marker and zero-fills the entry.
    /// - `OperatingSystem::IBMDOS200`: Uses `0x00` as the unused entry marker with `0xF6` as filler.
    /// - Other operating system variants default to `0x00` for both marker and filler.
    fn serialize_placeholder(os: &OperatingSystem) -> Vec<u8> {
        let (marker, filler) = match os {
            OperatingSystem::IBMDOS100 => (0xE5, 0xF6), // DOS 1.00: Deleted file marker, `0xF6` filler.
            OperatingSystem::IBMDOS110 => (0xE5, 0x00), // DOS 1.10: Deleted file marker, zero-filler.
            OperatingSystem::IBMDOS200 => (0x00, 0xF6), // DOS 2.00: Unused entry marker, `0xF6` filler.
            _ => (0x00, 0x00), // General case: `0x00` marker, `0x00` filler.
        };

        let mut bytes = Vec::with_capacity(32);
        bytes.push(marker); // First byte is the marker
        bytes.extend(std::iter::repeat(filler).take(31)); // Fill the remaining bytes
        bytes
    }
}