#[derive(Debug, PartialEq)]
pub enum FileSystemError {
    ClusterAlreadyAllocated,
    ClusterNotUsable,
    EmptyFileName,
    ExtensionTooLong,
    FileNameTooLong,
    InvalidCharInExt,
    InvalidCharInName,
    InvalidClusterIndex,
    InvalidPath,
    TooManyFileNameParts,
    WontShrinkAllocationTable,
}
