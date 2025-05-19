#[derive(Debug, PartialEq)]
pub enum FileSystemError {
    CannotAddParentlessEntry,
    CannotCreateDotfiles,
    ClusterAlreadyAllocated,
    ClusterNotUsable,
    DuplicateEntry,
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
