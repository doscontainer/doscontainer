#[cfg(test)]
mod tests {
    use crate::{allocationtable::AllocationTable, error::FileSystemError, fat12::Fat12, names::EntryName, FileSystem};

    use std::{path::PathBuf, str::FromStr};

    #[test]
    fn test_valid_filenames() {
        let cases = vec![
            ("README.TXT", "README", "TXT"),
            ("FILE1234.DOC", "FILE1234", "DOC"),
            ("HELLO", "HELLO", ""),
            ("A1B2C3D4.E1", "A1B2C3D4", "E1"),
        ];

        for (input, expected_name, expected_ext) in cases {
            let parsed = EntryName::from_str(input).expect("should be valid");
            assert_eq!(parsed.filename, expected_name);
            assert_eq!(parsed.extension, expected_ext);
        }
    }

    #[test]
    fn invalid_name_too_long() {
        let result = EntryName::from_str("ENDLESSLYLONGNAME.EXE");
        assert_eq!(result, Err(FileSystemError::FileNameTooLong));
    }

    #[test]
    fn invalid_ext_too_long() {
        let result = EntryName::from_str("FILE.FARTOOLONG");
        assert_eq!(result, Err(FileSystemError::ExtensionTooLong));
    }

    #[test]
    fn invalid_empty_filename() {
        let result = EntryName::from_str("");
        assert_eq!(result, Err(FileSystemError::EmptyFileName));
    }

    #[test]
    fn invalid_too_many_parts() {
        let result = EntryName::from_str("TOO.MANY.PARTS");
        assert_eq!(result, Err(FileSystemError::TooManyFileNameParts));
    }

    #[test]
    fn invalid_char_in_name() {
        let result = EntryName::from_str("BAD:NAME.TXT");
        assert_eq!(result, Err(FileSystemError::InvalidCharInName));
    }

    #[test]
    fn invalid_char_in_ext() {
        let result = EntryName::from_str("VALID.B?D");
        // The name part "VALID" is fine; the extension has bad characters.
        assert_eq!(result, Err(FileSystemError::InvalidCharInExt));
    }

    #[test]
    fn case_conversion() {
        let result = EntryName::from_str("mixed.Cas").expect("should be valid");
        assert_eq!(result.filename, "MIXED");
        assert_eq!(result.extension, "CAS");
    }

    #[test]
    fn wont_shrink_allocationtable() {
        let mut fat = AllocationTable::default();
        fat.set_cluster_count(50)
            .expect("Something BAD just happened!");
        let result = fat.set_cluster_count(40);
        assert_eq!(result, Err(FileSystemError::WontShrinkAllocationTable));
    }

    #[test]
    fn new_fat12() {
        let mut fat = Fat12::default();
        let path = PathBuf::from("/var/run/COMMAND.COM");
        assert!(fat.mkfile(path.as_path()).is_ok());
    }

    #[test]
    fn invalid_mkfile_fat12() {
        let mut fat = Fat12::default();
        let path = PathBuf::from("/var/run/COMMANDISFARTOOLONG.COM");
        assert_eq!(fat.mkfile(path.as_path()), Err(FileSystemError::FileNameTooLong));
    }
}
