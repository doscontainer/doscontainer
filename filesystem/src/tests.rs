#[cfg(test)]
mod tests {
    use crate::{
        allocationtable::AllocationTable,
        direntry::DirEntry,
        error::FileSystemError,
        fat12::Fat12,
        names::EntryName,
        pool::Pool,
        serializer::{ibmdos100::IbmDos100, Fat12Serializer},
        FileSystem,
    };
    use disk::{error::DiskError, volume::Volume, Disk};
    use operatingsystem::OperatingSystem;
    use std::{path::Path, str::FromStr};

    struct DummyDisk;

    /// Fake Disk implementation for testing Fat12
    impl Disk for DummyDisk {
        fn read_sector(&mut self, _lba: u64, _buffer: &mut [u8]) -> Result<(), DiskError> {
            Ok(())
        }
        fn write_sector(&mut self, _lba: u64, _buffer: &[u8]) -> Result<(), DiskError> {
            Ok(())
        }

        fn ibmwipe(&mut self) -> Result<(), DiskError> {
            Ok(())
        }

        fn sector_count(&self) -> u64 {
            312
        }

        fn sector_size(&self) -> disk::sectorsize::SectorSize {
            disk::sectorsize::SectorSize::S512
        }
    }

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
        assert_eq!(
            fat.set_cluster_count(50),
            Err(FileSystemError::WontShrinkAllocationTable)
        );
    }

    #[test]
    fn allocationtable_allocate_cluster() {
        let mut table = AllocationTable::default();
        assert!(table.set_cluster_count(340).is_ok());
        assert!(table.allocate(3, Some(4)).is_ok());
        assert!(!table.is_free(3).unwrap());
    }

    #[test]
    fn allocationtable_allocate_occupied() {
        let mut table = AllocationTable::default();
        assert!(table.set_cluster_count(340).is_ok());
        assert!(table.allocate(3, Some(4)).is_ok());
        assert_eq!(
            table.allocate(3, Some(4)),
            Err(FileSystemError::ClusterAlreadyAllocated)
        );
    }

    #[test]
    fn allocationtable_out_of_bounds() {
        let mut table = AllocationTable::default();
        assert!(table.set_cluster_count(340).is_ok());
        assert_eq!(
            table.allocate(350, None),
            Err(FileSystemError::InvalidClusterIndex)
        );
    }

    #[test]
    fn allocationtable_allocate_entry() {
        let mut table = AllocationTable::default();
        assert!(table.allocate_entry(16384).is_ok());
        assert_eq!(table.allocate_entry(16384).unwrap().len(), 32);
        assert_eq!(table.allocate_entry(16385).unwrap().len(), 33);
    }

    #[test]
    fn allocationtable_out_of_clusters() {
        let mut table = AllocationTable::default();
        assert_eq!(
            table.allocate_entry(327680),
            Err(FileSystemError::NotEnoughFreeClusters)
        );
    }

    #[test]
    fn allocationtable_too_large() {
        let mut table = AllocationTable::default();
        assert_eq!(
            table.set_cluster_count(5000),
            Err(FileSystemError::FatSizeTooLarge)
        );
    }

    #[test]
    fn new_fat12() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let mut fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            OperatingSystem::from_osshortname(&operatingsystem::OsShortName::IBMDOS100), None
        )
        .unwrap();
        assert!(fat.mkfile("/COMMAND.COM", &[0u8; 10], None).is_ok());
    }

    #[test]
    fn fat12_mkfile_with_length() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let data = [0u8; 4000];
        let mut fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ), None
        )
        .unwrap();
        assert!(fat.mkfile("/COMMAND.COM", &data, None).is_ok());
        assert!(fat.mkfile("/AUTOEXEC.BAT", &data, None).is_ok());
    }

    #[test]
    fn invalid_mkfile_fat12() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let mut fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ),None
        )
        .unwrap();
        assert_eq!(
            fat.mkfile("COMMANDISFARTOOLONG.COM", &[0u8; 512], None),
            Err(FileSystemError::FileNameTooLong)
        );
    }

    #[test]
    fn invalid_dotfiles_fat12() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let mut fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ), None
        )
        .unwrap();
        assert_eq!(
            fat.mkfile("..", &[0u8; 512], None),
            Err(FileSystemError::CannotCreateDotfiles)
        );
        assert_eq!(
            fat.mkfile(".", &[0u8; 512], None),
            Err(FileSystemError::CannotCreateDotfiles)
        );
        assert_eq!(
            fat.mkfile(".DOTFIL", &[0u8; 512], None),
            Err(FileSystemError::EmptyFileName)
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
            pool.entry_by_path(Path::new("DOS/EDIT.EXE"))
                .unwrap()
                .uuid()
                .clone(),
            edit_uuid
        );
    }

    #[test]
    fn fat12_mkdir() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let mut filesystem = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ), None
        )
        .unwrap();
        assert!(filesystem.mkdir("/DOS", 2, None).is_ok());
        let data = [0u8; 43221];
        assert!(filesystem.mkfile("/DOS/EDIT.EXE", &data, None).is_ok());
    }

    #[test]
    fn fat12_mkdir_hugedir() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let mut fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ), None
        )
        .unwrap();
        assert!(fat.mkdir("/DOS", 600, None).is_ok());
    }

    #[test]
    fn fat12_serialize_empty_ibmdos100() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ), None
        )
        .unwrap();
        let serializer = IbmDos100::serialize_fat12(fat.allocation_table()).unwrap();
        assert_eq!(
            serializer,
            vec![
                254, 255, 255, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }

    #[test]
    // Compares the actual FAT for an IBM PC-DOS 1.00 original boot disk with our serializer
    fn fat12_test_actual_pcdos100() {
        let mut disk = DummyDisk;
        let mut volume = Volume::new(&mut disk, 0, 340);
        let mut fat = Fat12::new(
            disk::sectorsize::SectorSize::S512,
            1,
            340,
            &mut volume,
            operatingsystem::OperatingSystem::from_osshortname(
                &operatingsystem::OsShortName::IBMDOS100,
            ), None
        )
        .unwrap();
        let os = OperatingSystem::from_vendor_version("ibm", "1.00").unwrap();
        fat.mkfile("IBMBIO.COM", os.iosys_bytes(), None).unwrap();
        fat.mkfile("IBMDOS.COM", os.msdossys_bytes(), None).unwrap();
        fat.mkfile("COMMAND.COM", os.commandcom_bytes(), None)
            .unwrap();
        let serializer = IbmDos100::serialize_fat12(fat.allocation_table()).unwrap();
        assert_eq!(
            serializer,
            vec![
                254, 255, 255, 3, 64, 0, 5, 240, 255, 7, 128, 0, 9, 160, 0, 11, 192, 0, 13, 224, 0,
                15, 0, 1, 17, 32, 1, 255, 79, 1, 21, 96, 1, 23, 128, 1, 25, 240, 255, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
                0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0
            ]
        );
    }
}
