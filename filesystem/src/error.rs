use std::fmt;

#[derive(Debug, PartialEq)]
pub enum FileSystemError {
    /// The allocation table is full, and no more clusters can be allocated.
    AllocationTableFull,

    /// Attempted to allocate or access a cluster that is not free.
    ClusterNotFree { index: usize },

    /// Accessed a cluster index that is out of bounds.
    ClusterOutOfBounds { index: usize, cluster_count: usize },

    /// Day is out of range for FAT data structures.
    DayOutOfRange,

    /// The directory with the specified name already exists.
    DirectoryAlreadyExists { name: String },

    /// Generic error for duplicate entries.
    DuplicateEntry,

    /// Attempted to access an entry that does not exist.
    EntryDoesNotExist,

    /// Entry of this type can not have children.
    EntryCanNotHaveChildren,

    /// Attempted to set an extension on a type that is not supported.
    ExtensionNotSupported,

    /// Attempted to create a file that already exists.
    FileAlreadyExists,

    /// The file extension contains an invalid character.
    InvalidCharacterInExtension,

    /// The file name contains an invalid character.
    InvalidCharacterInName,

    /// The date is invalid
    InvalidDate,

    /// The disk type is invalid (in combination with other factors)
    InvalidDiskType,

    /// Invalid entry for the operation to work on
    InvalidEntryType,

    /// The Operating System is invalid
    InvalidOperatingSystem,

    /// Invalid path given
    InvalidPath,

    /// The root directory type provided is invalid for the operation.
    InvalidRootType,

    /// Time is invalid
    InvalidTime,

    /// Missing date/time
    MissingDateTime,

    /// Month is out of range for FAT data structures.
    MonthOutOfRange,

    /// No clusters allocated to this entry
    NoAllocatedClusters,

    /// Found an orphaned directory or file entry with no valid parent.
    OrphanedEntry { path: String },

    /// Root directory does not exist.
    RootDirectoryDoesNotExist,

    /// A volume may have 0 or 1 labels, no more than that.
    TooManyVolumeLabels,

    /// Unsupported disk type. The disk may be *valid*,
    /// it's just being used in a way that's unsupported.
    UnsupportedDiskType,

    /// Unsupported operating system in a specific context
    UnsupportedOperatingSystem,

    /// The volume already has a label, and a new one cannot be set.
    VolumeLabelAlreadyExists,

    /// Error with the parentage of the `VolumeLabel`: parent must be the root dir
    VolumeLabelParentError,

    /// The given year is out of range for the FAT data structures.
    YearOutOfRange,
}

impl std::fmt::Display for FileSystemError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileSystemError::AllocationTableFull => {
                write!(f, "Allocation table full. All clusters are allocated")
            }
            FileSystemError::ClusterNotFree { index } => {
                write!(f, "Cluster at index {index} is not free")
            }
            FileSystemError::ClusterOutOfBounds {
                index,
                cluster_count,
            } => {
                write!(
                    f,
                    "Cluster index {index} is out of bounds (max: {cluster_count})"
                )
            }
            FileSystemError::DayOutOfRange => {
                write!(f, "Day is out of range.")
            }
            FileSystemError::DirectoryAlreadyExists { name } => {
                write!(f, "Directory '{name}' already exists")
            }
            FileSystemError::DuplicateEntry => {
                write!(f, "Duplicate entry.")
            }
            FileSystemError::EntryCanNotHaveChildren => {
                write!(f, "Entries of this type can not have children.")
            }
            FileSystemError::EntryDoesNotExist => {
                write!(f, "Entry does not exist")
            }
            FileSystemError::ExtensionNotSupported => {
                write!(f, "Setting an extension on this type is not supported.")
            }
            FileSystemError::FileAlreadyExists => {
                write!(f, "File already exists")
            }
            FileSystemError::InvalidCharacterInExtension => {
                write!(f, "Invalid character in extension.")
            }
            FileSystemError::InvalidCharacterInName => {
                write!(f, "Invalid character in name.")
            }
            FileSystemError::InvalidDate => {
                write!(f, "Invalid date.")
            }
            FileSystemError::InvalidDiskType => {
                write!(f, "Invalid disk type.")
            }
            FileSystemError::InvalidEntryType => {
                write!(f, "Invalid entry type for this operation.")
            }
            FileSystemError::InvalidOperatingSystem => {
                write!(f, "Invalid operating system.")
            }
            FileSystemError::InvalidRootType => {
                write!(f, "Invalid root type.")
            }
            FileSystemError::InvalidPath => {
                write!(f, "Invalid path.")
            }
            FileSystemError::InvalidTime => {
                write!(f, "Inavlid time.")
            }
            FileSystemError::MissingDateTime => {
                write!(f, "Missing date/time.")
            }
            FileSystemError::MonthOutOfRange => {
                write!(f, "Month is out of range.")
            }
            FileSystemError::NoAllocatedClusters => {
                write!(f, "No clusters allocated to this entry.")
            }
            FileSystemError::OrphanedEntry { path } => {
                write!(f, "Orphaned entry at path '{path}'")
            }
            FileSystemError::RootDirectoryDoesNotExist => {
                write!(f, "Root directory does not exist.")
            }
            FileSystemError::TooManyVolumeLabels => {
                write!(f, "Too many volume labels present.")
            }
            FileSystemError::UnsupportedDiskType => {
                write!(f, "Unsupported disk type.")
            }
            FileSystemError::UnsupportedOperatingSystem => {
                write!(f, "Unsupported operating system.")
            }
            FileSystemError::VolumeLabelAlreadyExists => {
                write!(f, "Volume label already exists")
            }
            FileSystemError::VolumeLabelParentError => {
                write!(f, "Volume label's parent must be the root directory.")
            }
            FileSystemError::YearOutOfRange => {
                write!(f, "Year is out of range for FAT.")
            }
        }
    }
}

impl std::error::Error for FileSystemError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
