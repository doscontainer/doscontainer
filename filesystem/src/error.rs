#[derive(Debug, PartialEq)]
pub enum FileSystemError {
    CannotAddParentlessEntry,
    ClusterAlreadyAllocated,
    ClusterNotUsable,
    EntryCannotHaveChildren,
    EmptyFileName,
    ExtensionTooLong,
    FileNameTooLong,
    InvalidCharInExt,
    InvalidCharInName,
    InvalidClusterIndex,
    InvalidPath,
    ParentNotFound,
    TooManyFileNameParts,
    WontShrinkAllocationTable,
}
