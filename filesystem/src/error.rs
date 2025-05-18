#[derive(Debug, PartialEq)]
pub enum FileSystemError {
    EmptyFileName,
    ExtensionTooLong,
    FileNameTooLong,
    InvalidCharInExt,
    InvalidCharInName,
    TooManyFileNameParts,
}
