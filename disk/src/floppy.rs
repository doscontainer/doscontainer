use std::{
    fmt,
    fs::{File, OpenOptions},
    path::PathBuf,
};

use crate::{disktype::DiskType, error::DiskError, geometry::Geometry, Disk};

#[derive(Debug)]
pub struct Floppy {
    disktype: DiskType,
    file: File,
}

impl fmt::Display for Floppy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Floppy disk type: {}", self.disktype)
    }
}

impl Disk for Floppy {
    fn geometry(&self) -> Result<Geometry, DiskError> {
        let geometry = match self.disktype {
            DiskType::F35_1440 => Geometry::new(80, 2, 18)?,
            DiskType::F35_2880 => Geometry::new(80, 2, 36)?,
            DiskType::F35_720 => Geometry::new(80, 2, 9)?,
            DiskType::F525_1200 => Geometry::new(80, 2, 15)?,
            DiskType::F525_360 => Geometry::new(40, 2, 9)?,
            DiskType::F525_320 => Geometry::new(40, 2, 8)?,
            DiskType::F525_180 => Geometry::new(40, 1, 9)?,
            DiskType::F525_160 => Geometry::new(40, 1, 8)?,
            DiskType::HardDisk => return Err(DiskError::UnsupportedDiskType),
        };
        Ok(geometry)
    }

    fn sector_count(&self) -> Result<usize, DiskError> {
        let geometry = self.geometry()?;
        Ok(geometry.get_cylinders() * geometry.get_heads() * geometry.get_sectors())
    }

    fn sector_size(&self) -> Result<usize, DiskError> {
        Ok(self.disktype.sector_size())
    }

    fn file(&self) -> &File {
        &self.file
    }

    fn disktype(&self) -> DiskType {
        self.disktype.clone()
    }
}

impl Floppy {
    pub fn new(disktype: DiskType, filename: PathBuf) -> Result<Floppy, DiskError> {
        // Determine the sector size and sector count based on the disk type
        let (sector_size, sector_count) = match disktype {
            DiskType::F35_1440
            | DiskType::F35_2880
            | DiskType::F35_720
            | DiskType::F525_1200
            | DiskType::F525_160
            | DiskType::F525_180
            | DiskType::F525_320
            | DiskType::F525_360 => (512, disktype.sector_count().unwrap()),
            _ => return Err(DiskError::InvalidDiskType),
        };

        let new_file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(filename)
            .map_err(DiskError::IoError)?;
        new_file.set_len(sector_size as u64 * sector_count as u64)?;

        Ok(Floppy {
            file: new_file,
            disktype,
        })
    }
}