use chrono::{DateTime, Datelike, Local, Timelike};
use operatingsystem::OperatingSystem;
use uuid::Uuid;

use crate::{attributes::Attributes, error::FileSystemError};

pub struct DirEntry {
    entry_type: DirEntryType,
    extension: Option<String>,
    attributes: Attributes,
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

/// The default DirEntry is a Directory
impl Default for DirEntry {
    fn default() -> Self {
        Self::new(DirEntryType::Directory)
    }
}

impl DirEntry {
    /// Generate a new `DirEntry` instance based on the given `DirEntryType`.
    ///
    /// # Arguments
    /// * `entrytype` - Specifies the type of directory entry to create, such as a file, directory, or placeholder.
    ///
    /// # Returns
    /// A `DirEntry` instance corresponding to the provided `DirEntryType`.
    pub fn new(entrytype: DirEntryType) -> Self {
        match entrytype {
            DirEntryType::EmptyPlaceholder => Self::new_placeholder(),
            DirEntryType::Directory => Self::new_directory(),
            DirEntryType::File => Self::new_file(),
            DirEntryType::SysFile => Self::new_sysfile(),
            DirEntryType::VolumeLabel => Self::new_volumelabel(),
        }
    }

    /// Generate a new `DirEntry` instance representing a directory.
    fn new_directory() -> Self {
        DirEntry {
            entry_type: DirEntryType::Directory,
            extension: None,
            attributes: Attributes::from_preset(crate::attributes::AttributesPreset::Directory),
            file_size: 0,
            last_modified_time: chrono::Local::now(),
            name: None,
            parent: None,
            allocated_clusters: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

    /// Generate a new `DirEntry` instance representing an empty placeholder.
    fn new_placeholder() -> Self {
        DirEntry {
            entry_type: DirEntryType::EmptyPlaceholder,
            extension: None,
            attributes: Attributes::from_preset(
                crate::attributes::AttributesPreset::EmptyPlaceholder,
            ),
            file_size: 0,
            last_modified_time: chrono::Local::now(),
            name: None,
            parent: None,
            allocated_clusters: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

    /// Generate a new `DirEntry` instance representing a regular file.
    fn new_file() -> Self {
        DirEntry {
            entry_type: DirEntryType::File,
            extension: None,
            attributes: Attributes::from_preset(crate::attributes::AttributesPreset::RegularFile),
            file_size: 0,
            last_modified_time: chrono::Local::now(),
            name: None,
            parent: None,
            allocated_clusters: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

    /// Generate a new `DirEntry` instance representing a system file.
    fn new_sysfile() -> Self {
        DirEntry {
            entry_type: DirEntryType::SysFile,
            extension: None,
            attributes: Attributes::from_preset(crate::attributes::AttributesPreset::SystemFile),
            file_size: 0,
            last_modified_time: chrono::Local::now(),
            name: None,
            parent: None,
            allocated_clusters: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

    /// Generate a new `DirEntry` instance representing a volume label.
    fn new_volumelabel() -> Self {
        DirEntry {
            entry_type: DirEntryType::VolumeLabel,
            extension: None,
            attributes: Attributes::from_preset(crate::attributes::AttributesPreset::VolumeLabel),
            file_size: 0,
            last_modified_time: chrono::Local::now(),
            name: None,
            parent: None,
            allocated_clusters: Vec::new(),
            uuid: Uuid::new_v4(),
        }
    }

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
            DirEntryType::File => Ok(self.serialize_file(os)?),
            DirEntryType::SysFile => Ok(self.serialize_sysfile(os)?),
            DirEntryType::VolumeLabel => Ok(self.serialize_volume_label()?),
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
                output.push(self.attributes.as_byte());
                output.extend(std::iter::repeat(0).take(10));
                output.extend(self.startcluster_as_bytes()?); // 2 bytes for start cluster
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
    fn serialize_file(&self, os: &OperatingSystem) -> Result<Vec<u8>, FileSystemError> {
        let mut output = Vec::with_capacity(32); // Preallocate 32 bytes for efficiency.

        // Add common fields (name, extension, type).
        output.extend(self.name_as_bytes()); // 8 bytes for name.
        output.extend(self.ext_as_bytes()); // 3 bytes for extension.
        output.push(self.attributes.as_byte()); // 1 byte for type.

        // Handle date/time serialization
        let (fat_date, fat_time) = match self.last_modified() {
            Ok((date, time)) => (date, time),
            Err(_) => return Err(FileSystemError::MissingDateTime),
        };

        let serialize_date_time = |reserved_bytes: usize, fat_date: u16, fat_time: u16| {
            let mut temp_output = Vec::with_capacity(reserved_bytes + 4);
            temp_output.extend(std::iter::repeat(0).take(reserved_bytes)); // Reserved bytes
            temp_output.extend(fat_time.to_le_bytes()); // 2 bytes for time
            temp_output.extend(fat_date.to_le_bytes()); // 2 bytes for date
            temp_output
        };

        // Match logic based on the operating system.
        match os {
            // IBM PC-DOS 1.10 and 2.00: Record date and time.
            OperatingSystem::IBMDOS110 | OperatingSystem::IBMDOS200 => {
                output.extend(serialize_date_time(10, fat_date, fat_time)); // Reserved 10 bytes for DOS 2.00
            }

            // IBM PC-DOS 1.00: Only record the date (no time field).
            OperatingSystem::IBMDOS100 => {
                let mut temp_output = Vec::with_capacity(14);
                temp_output.extend(std::iter::repeat(0).take(12)); // Reserved 12 bytes
                temp_output.extend(fat_date.to_le_bytes()); // 2 bytes for date
                output.extend(temp_output);
            }

            // Other operating systems: Use default serialization.
            _ => {
                output.extend(serialize_date_time(10, fat_date, fat_time)); // Default serialization for other OS
            }
        }

        // Add start cluster and file size.
        output.extend(self.startcluster_as_bytes()?); // 2 bytes for start cluster.
        output.extend(&(self.file_size as u32).to_le_bytes()); // 4 bytes for file size.

        Ok(output)
    }

    /// Serialize a system file. This is just a file, with the sole exception of its attributes.
    fn serialize_sysfile(&self, os: &OperatingSystem) -> Result<Vec<u8>, FileSystemError> {
        let mut output = self.serialize_file(os)?;
        // The attribute byte lives at byte 11 in the serialized output
        output[11] =
            Attributes::from_preset(crate::attributes::AttributesPreset::SystemFile).as_byte();
        Ok(output)
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

    /// Serializes a `DirEntry` of type `VolumeLabel` to a byte array in the FAT file system format.
    ///
    /// This method takes a `DirEntry` representing a volume label and converts it into a 32-byte array
    /// suitable for inclusion in the FAT directory structure. Volume label entries have specific characteristics
    /// that differ from regular file entries:
    ///
    /// - The name is limited to 8 characters and the extension to 3 characters, with the extension padded
    ///   with spaces (if necessary).
    /// - The attributes byte is set to `0x08`, which indicates a volume label.
    /// - The file size is always `0` (as volume labels are not files).
    /// - The start cluster is usually set to `0` (reserved for volume labels).
    /// - The modification time and date are set to `0` as volume labels do not have modification times.
    ///
    /// # Returns
    ///
    /// Returns a `Vec<u8>` representing the 32-byte serialized FAT directory entry for the volume label.
    fn serialize_volume_label(&self) -> Result<Vec<u8>, FileSystemError> {
        // Initialize a vector to hold the serialized bytes
        let mut output = Vec::with_capacity(32); // A FAT directory entry is always 32 bytes

        // Volume labels are always 8.3 format, so we limit the name to 8 characters and extension to 3
        output.extend(self.name_as_bytes()); // 8 bytes for name
        output.extend(self.ext_as_bytes()); // 3 bytes for extension

        // Set the attributes byte for volume label (0x08 is used for volume labels in FAT)
        output.push(0x08); // Volume label attribute

        // Reserved bytes (10 bytes reserved for use by FAT)
        output.extend(std::iter::repeat(0).take(10));

        // Volume labels don't have a modification time or date, so we use 0 for both.
        output.extend(std::iter::repeat(0).take(4)); // 2 bytes for time and 2 for date

        // Volume labels have no file size, so we set the start cluster to 0 and size to 0
        output.extend(self.startcluster_as_bytes()?); // 2 bytes for start cluster (usually 0 for volume labels)
        output.extend(&[0, 0, 0, 0]); // 4 bytes for file size, set to 0

        Ok(output)
    }

    /// Helper function to convert a string into a FAT-compatible byte array of a given length.
    ///
    /// - Non-ASCII characters and invalid FAT characters are removed from the input.
    /// - The string is truncated to the specified length.
    /// - All characters are converted to uppercase ASCII.
    /// - The resulting byte array is padded with spaces (`b' '`) if its length is less than the specified length.
    ///
    /// # Parameters
    /// - `input`: The input string to convert.
    /// - `length`: The length of the resulting byte array.
    ///
    /// # Returns
    /// A `Vec<u8>` containing the FAT-compatible byte array representation of the input string.
    fn to_fat_bytes(input: &str, length: usize) -> Vec<u8> {
        // Define the set of valid FAT characters
        const VALID_FAT_CHARACTERS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!#$%&'()-@^_`{}~";

        // Create a buffer, filtering out invalid characters, converting to uppercase, and limiting to the specified length
        let mut output: Vec<u8> = input
            .chars()
            .filter(|c| c.is_ascii()) // Keep only ASCII characters
            .map(|c| c.to_ascii_uppercase() as u8) // Convert to uppercase
            .filter(|b| VALID_FAT_CHARACTERS.contains(b)) // Filter out invalid FAT characters
            .take(length) // Limit to the specified length
            .collect();

        // Pad with spaces to ensure the length is exactly the specified length
        output.resize(length, b' ');
        output
    }

    /// Convert the filename part of the entry to a set of bytes
    fn name_as_bytes(&self) -> Vec<u8> {
        let input_str = self.name.as_deref().unwrap_or("");
        Self::to_fat_bytes(input_str, 8)
    }

    /// Convert the extension part of the entry to a set of bytes
    fn ext_as_bytes(&self) -> Vec<u8> {
        let input_str = self.extension.as_deref().unwrap_or("");
        Self::to_fat_bytes(input_str, 3)
    }

    /// Converts the first allocated cluster to a byte array (Little Endian).
    ///
    /// This method assumes that `allocated_clusters` contains at least one element.
    /// If the cluster list is empty, the method will return an error or a default value (e.g., 0).
    ///
    /// # Returns
    /// - A `Vec<u8>` representing the start cluster in Little Endian byte order.
    fn startcluster_as_bytes(&self) -> Result<Vec<u8>, FileSystemError> {
        if let Some(start_cluster) = self.allocated_clusters.first() {
            Ok(start_cluster.to_le_bytes().to_vec())
        } else {
            Err(FileSystemError::NoAllocatedClusters) // Handle case with no allocated clusters
        }
    }

    /// Converts the last modified time to the FAT date and time format (16-bit values).
    ///
    /// This method takes the `last_modified_time` of the entry, converts it into the
    /// FAT date and time formats, and returns them as a tuple of 16-bit values.
    ///
    /// The FAT date format consists of:
    /// - 7 bits for the year (relative to 1980, with a range of 0 to 127)
    /// - 4 bits for the month (1-12)
    /// - 5 bits for the day (1-31)
    ///
    /// The FAT time format consists of:
    /// - 5 bits for the hour (0-23)
    /// - 6 bits for the minute (0-59)
    /// - 5 bits for the second (0-29), which is the number of 2-second intervals
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing a tuple `(fat_date, fat_time)` where:
    /// - `fat_date` is the 16-bit date value (packed as year, month, and day).
    /// - `fat_time` is the 16-bit time value (packed as hour, minute, and second).
    ///
    /// If any of the date or time values are out of range, an appropriate error is returned.
    pub fn last_modified(&self) -> Result<(u16, u16), FileSystemError> {
        // Get year, month, and day
        let year = self.last_modified_time.year() - 1980; // Years since 1980
        let month = self.last_modified_time.month(); // 1-12
        let day = self.last_modified_time.day(); // 1-31

        // Validate values
        if !(0..=127).contains(&year) {
            return Err(FileSystemError::YearOutOfRange);
        }
        if !(1..=12).contains(&month) {
            return Err(FileSystemError::MonthOutOfRange);
        }
        if !(1..=31).contains(&day) {
            return Err(FileSystemError::DayOutOfRange);
        }

        // Create FAT date (16 bits)
        let fat_date = ((year as u16) << 9) | ((month as u16) << 5) | (day as u16);

        // Get hours, minutes, and seconds
        let hour = self.last_modified_time.hour(); // 0-23
        let minute = self.last_modified_time.minute(); // 0-59
        let second = self.last_modified_time.second() / 2; // Seconds divided by 2 (0-29)

        // Create FAT time (16 bits)
        let fat_time = ((hour as u16) << 11) | ((minute as u16) << 5) | (second as u16);

        Ok((fat_date, fat_time))
    }

    pub fn set_parent(&mut self, parent: Uuid) {
        self.parent = Some(parent);
    }

    pub fn entry_type(&self) -> DirEntryType {
        self.entry_type
    }

    pub fn parent(&self) -> Option<Uuid> {
        self.parent
    }

    pub fn id(&self) -> Uuid {
        self.uuid
    }

    pub fn name(&self) -> Option<&String> {
        self.name.as_ref()
    }

    pub fn normalized_name(&self) -> Option<String> {
        self.name.as_ref().map(|name| {
            match self.extension.as_ref() {
                Some(ext) => format!("{}.{}", name, ext),
                None => name.clone(),
            }
        })
    }
    
    pub fn set_allocated_clusters(&mut self, clusters: &[usize]) {
        self.allocated_clusters = clusters.to_vec();
    }
}
