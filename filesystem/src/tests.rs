#[cfg(test)]
mod tests {
    use crate::{
        allocationtable::AllocationTable, direntry::DirEntry, error::FileSystemError, fat12::Fat12,
        names::EntryName, pool::Pool, FileSystem,
    };

    use std::{ops::Deref, path::PathBuf, str::FromStr};

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
        assert_eq!(
            fat.mkfile(path.as_path()),
            Err(FileSystemError::FileNameTooLong)
        );
    }

    #[test]
    fn pool_prevent_duplicates() {
        let mut pool = Pool::default();
        // Initial entry under root
        let mut entry = DirEntry::new_file("COMMAND.COM").unwrap();
        entry.set_parent(pool.root_entry().unwrap());
        // This creates a new entry with the same name under the same parent.
        let mut duplicate = DirEntry::new_file("COMMAND.COM").unwrap();
        duplicate.set_parent(pool.root_entry().unwrap());
        // Adding the first entry should succeed
        assert!(pool.add_entry(entry).is_ok());
        // The duplicate should complain about being a duplicate.
        assert_eq!(
            pool.add_entry(duplicate),
            Err(FileSystemError::DuplicateEntry)
        );
    }

    #[test]
    fn pool_parentless_entry() {
        let mut pool = Pool::default();
        let entry = DirEntry::new_file("COMMAND.COM").unwrap();
        assert_eq!(
            pool.add_entry(entry),
            Err(FileSystemError::CannotAddParentlessEntry)
        );
    }

    #[test]
    fn pool_add_child_to_file() {
        let mut pool = Pool::default();
        let mut entry = DirEntry::new_file("COMMAND.COM").unwrap();
        entry.set_parent(pool.root_entry().unwrap());
        let mut child = DirEntry::new_file("AUTOEXEC.BAT").unwrap();
        child.set_parent(&entry);
        assert!(pool.add_entry(entry).is_ok());
        assert_eq!(
            pool.add_entry(child),
            Err(FileSystemError::EntryCannotHaveChildren)
        );
    }

    #[test]
    fn pool_valid_subdir_entry() {
        let mut pool = Pool::default();
        let mut dos = DirEntry::new_directory("DOS").unwrap();
        dos.set_parent(pool.root_entry().unwrap());
        let mut edit_exe = DirEntry::new_file("EDIT.EXE").unwrap();
        edit_exe.set_parent(&dos);
        // Creating a DOS subdir should work
        assert!(pool.add_entry(dos).is_ok());
        // Adding EDIT.EXE under the DOS directory should also work.
        assert!(pool.add_entry(edit_exe).is_ok());
    }

    #[test]
    fn pool_retrieve_entry_by_name() {
        let mut pool = Pool::default();
        let mut dos = DirEntry::new_directory("DOS").unwrap();
        dos.set_parent(pool.root_entry().unwrap());
        let dos_uuid = dos.uuid().clone();
        let mut edit_exe = DirEntry::new_file("EDIT.EXE").unwrap();
        let edit_uuid = edit_exe.uuid().clone();
        edit_exe.set_parent(&dos);
        // Creating a DOS subdir should work
        assert!(pool.add_entry(dos).is_ok());
        // Adding EDIT.EXE under the DOS directory should also work.
        assert!(pool.add_entry(edit_exe).is_ok());
        let dos_retrieved = pool.entry(&dos_uuid).unwrap();
        let edit_retrieved = pool.entry(&edit_uuid).unwrap();
        let retrieved = pool.entry_by_name("EDIT.EXE", dos_retrieved);
        assert!(retrieved.is_ok());
        assert_eq!(edit_retrieved, retrieved.unwrap().unwrap());
    }

    #[test]
    fn pool_entry_by_path() {
        let mut pool = Pool::default();
        let mut dos = DirEntry::new_directory("DOS").unwrap();
        dos.set_parent(pool.root_entry().unwrap());
        let mut edit_exe = DirEntry::new_file("EDIT.EXE").unwrap();
        edit_exe.set_parent(&dos);
        let edit_uuid = edit_exe.uuid().clone();
        // Creating a DOS subdir should work
        assert!(pool.add_entry(dos).is_ok());
        // Adding EDIT.EXE under the DOS directory should also work.
        assert!(pool.add_entry(edit_exe).is_ok());
        assert_eq!(
            pool.entry_by_path("DOS/EDIT.EXE")
                .unwrap()
                .uuid()
                .clone(),
            edit_uuid
        );
    }
}
