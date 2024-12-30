use bpb::BPB;
use disk::disktype::DiskType;
use error::OsError;

pub mod bpb;
pub mod error;

/// The OperatingSystem enum holds specific fragments of
/// code and data that apply only to a particular operating system
#[derive(Clone, Debug, PartialEq)]
pub enum OperatingSystem {
    IBMDOS100,
    IBMDOS110,
    IBMDOS200,
    NONE,
    UNKNOWN,
}

impl OperatingSystem {
    /// Retrieve the jump code at the start of the boot sector
    pub fn get_jumpcode(&self) -> [u8; 3] {
        match self {
            // This does JMP 0x2F. What the 0x14 does is a mystery, but it's what the original OS puts
            // at this position, so we do too.
            Self::IBMDOS100 => [0xEB, 0x2F, 0x14],
            Self::IBMDOS110 => [0xEB, 0x27, 0x90],
            Self::IBMDOS200 => [0x00, 0x00, 0x00],
            Self::NONE => [0x00, 0x00, 0x00],
            Self::UNKNOWN => [0x00, 0x00, 0x00],
        }
    }

    pub fn from_str(os: &str) -> Self {
        match os.to_ascii_uppercase().as_str() {
            "IBMDOS100" => Self::IBMDOS100,
            "IBMDOS110" => Self::IBMDOS110,
            "IBMDOS200" => Self::IBMDOS200,
            "NONE" => Self::NONE,
            _ => Self::UNKNOWN,
        }
    }

    /// Returns a friendly name for the enum variant. Use for display purposes.
    pub fn get_friendlyname(&self) -> String {
        match self {
            Self::IBMDOS100 => "IBM PC-DOS 1.00".to_string(),
            Self::IBMDOS110 => "IBM PC-DOS 1.10".to_string(),
            Self::IBMDOS200 => "IBM PC-DOS 2.00".to_string(),
            Self::UNKNOWN => "Unknown operating system".to_string(),
            Self::NONE => "No operating system".to_string(),
        }
    }

    /// Return the filename this OS uses for the MSDOS.SYS equivalent system file.
    pub fn get_msdossys(&self) -> String {
        match self {
            Self::IBMDOS100 => "IBMDOS.COM".to_string(),
            Self::IBMDOS110 => "IBMDOS.COM".to_string(),
            Self::IBMDOS200 => "IBMDOS.COM".to_string(),
            _ => "MSDOS.SYS".to_string(),
        }
    }

    /// Return the filename this OS uses for the IO.SYS equivalent system file.
    pub fn get_iosys(&self) -> String {
        match self {
            Self::IBMDOS100 => "IBMBIO.COM".to_string(),
            Self::IBMDOS110 => "IBMBIO.COM".to_string(),
            Self::IBMDOS200 => "IBMBIO.COM".to_string(),
            _ => "IO.SYS".to_string(),
        }
    }

    /// Return the default download URL for an operating system zipfile
    pub fn get_download_url(&self) -> String {
        match self {
            Self::IBMDOS100 => {
                "https://dosk8s-dist.area536.com/ibm-pc-dos-100-bootstrap.zip".to_string()
            }
            Self::IBMDOS110 => {
                "https://dosk8s-dist.area536.com/ibm-pc-dos-110-bootstrap.zip".to_string()
            }
            Self::IBMDOS200 => {
                "https://dosk8s-dist.area536.com/ibm-pc-dos-200-bootstrap.zip".to_string()
            }
            _ => "https://dosk8s-dist.area536.com/none.zip".to_string(), // Not working yet!
        }
    }

    /// Return the filename this OS uses for the COMMAND.COM equivalent system file.
    pub fn get_commandcom(&self) -> String {
        "COMMAND.COM".to_string()
    }

    /// Returns the full boot sector / volume boot record for the OS variant specified
    pub fn get_bootsector(&self, disktype: &DiskType) -> Result<Option<Vec<u8>>, OsError> {
        match self {
            // IBM PC-DOS 1.00 is special: there was only one disk format, the entire sector is hard-coded
            // because no variants were ever supported on real sytems so we just return the raw bytes as
            // lifted from an ancient installation disk     here.
            Self::IBMDOS100 => Ok(Some(vec![
                0xEB, 0x2F, 0x14, 0x00, 0x00, 0x00, 0x60, 0x00, 0x20, 0x37, 0x2D, 0x4D, 0x61, 0x79,
                0x2D, 0x38, 0x31, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xFA, 0x8C, 0xC8, 0x8E, 0xD8, 0xBA, 0x00,
                0x00, 0x8E, 0xD2, 0xBC, 0x00, 0x7C, 0xFB, 0xA1, 0x06, 0x7C, 0x8E, 0xD8, 0x8E, 0xC0,
                0xBA, 0x00, 0x00, 0x8B, 0xC2, 0xCD, 0x13, 0x72, 0x41, 0xE8, 0x58, 0x00, 0x72, 0xFB,
                0x2E, 0x8B, 0x0E, 0x02, 0x7C, 0x51, 0xBB, 0x00, 0x00, 0x33, 0xD2, 0xB9, 0x08, 0x00,
                0xBE, 0x01, 0x00, 0x56, 0xB0, 0x01, 0xB4, 0x02, 0xCD, 0x13, 0x72, 0x22, 0x5E, 0x58,
                0xE8, 0xE7, 0x00, 0x2B, 0xC6, 0x74, 0x14, 0xFE, 0xC5, 0xB1, 0x01, 0xBE, 0x08, 0x00,
                0x3B, 0xC6, 0x73, 0x04, 0x8B, 0xF0, 0xEB, 0x01, 0x96, 0x56, 0x50, 0xEB, 0xDD, 0x2E,
                0xFF, 0x2E, 0x04, 0x7C, 0xBE, 0x44, 0x7D, 0xB8, 0x42, 0x7D, 0x50, 0x32, 0xFF, 0xAC,
                0x24, 0x7F, 0x74, 0x0B, 0x56, 0xB4, 0x0E, 0xBB, 0x07, 0x00, 0xCD, 0x10, 0x5E, 0xEB,
                0xF0, 0xC3, 0xBB, 0x00, 0x00, 0xB9, 0x04, 0x00, 0xB8, 0x01, 0x02, 0xCD, 0x13, 0x1E,
                0x72, 0x34, 0x8C, 0xC8, 0x8E, 0xD8, 0xBF, 0x00, 0x00, 0xB9, 0x0B, 0x00, 0x26, 0x80,
                0x0D, 0x20, 0x26, 0x80, 0x8D, 0x20, 0x00, 0x20, 0x47, 0xE2, 0xF3, 0xBF, 0x00, 0x00,
                0xBE, 0x76, 0x7D, 0xB9, 0x0B, 0x00, 0xFC, 0xF3, 0xA6, 0x75, 0x0F, 0xBF, 0x20, 0x00,
                0xBE, 0x82, 0x7D, 0xB9, 0x0B, 0x00, 0xF3, 0xA6, 0x75, 0x02, 0x1F, 0xC3, 0xBE, 0xF9,
                0x7C, 0xE8, 0xA5, 0xFF, 0xB4, 0x00, 0xCD, 0x16, 0x1F, 0xF9, 0xC3, 0x0D, 0x0A, 0x4E,
                0x6F, 0x6E, 0x2D, 0x53, 0x79, 0x73, 0x74, 0x65, 0x6D, 0x20, 0x64, 0x69, 0x73, 0x6B,
                0x20, 0x6F, 0x72, 0x20, 0x64, 0x69, 0x73, 0x6B, 0x20, 0x65, 0x72, 0x72, 0x6F, 0xF2,
                0x0D, 0x0A, 0x52, 0x65, 0x70, 0x6C, 0x61, 0x63, 0x65, 0x20, 0x61, 0x6E, 0x64, 0x20,
                0x73, 0x74, 0x72, 0x69, 0x6B, 0x65, 0x20, 0x61, 0x6E, 0x79, 0x20, 0x6B, 0x65, 0x79,
                0x20, 0x77, 0x68, 0x65, 0x6E, 0x20, 0x72, 0x65, 0x61, 0x64, 0xF9, 0x0D, 0x0A, 0x00,
                0xCD, 0x18, 0x0D, 0x0A, 0x44, 0x69, 0x73, 0x6B, 0x20, 0x42, 0x6F, 0x6F, 0x74, 0x20,
                0x66, 0x61, 0x69, 0x6C, 0x75, 0x72, 0xE5, 0x0D, 0x0A, 0x00, 0x50, 0x52, 0x8B, 0xC6,
                0xBF, 0x00, 0x02, 0xF7, 0xE7, 0x03, 0xD8, 0x5A, 0x58, 0xC3, 0x52, 0x6F, 0x62, 0x65,
                0x72, 0x74, 0x20, 0x4F, 0x27, 0x52, 0x65, 0x61, 0x72, 0x20, 0x69, 0x62, 0x6D, 0x62,
                0x69, 0x6F, 0x20, 0x20, 0x63, 0x6F, 0x6D, 0xB0, 0x69, 0x62, 0x6D, 0x64, 0x6F, 0x73,
                0x20, 0x20, 0x63, 0x6F, 0x6D, 0xB0, 0xC9, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ])),
            Self::IBMDOS110 => Ok(Some(vec![
                0xEB, 0x27, 0x90, 0x03, 0x01, 0x14, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xCD, 0x19, 0xFA,
                0x8C, 0xC8, 0x8E, 0xD8, 0x33, 0xD2, 0x8E, 0xD2, 0xBC, 0x00, 0x7C, 0xFB, 0xB8, 0x60,
                0x00, 0x8E, 0xD8, 0x8E, 0xC0, 0x33, 0xD2, 0x8B, 0xC2, 0xCD, 0x13, 0x72, 0x69, 0xE8,
                0x85, 0x00, 0x72, 0xDD, 0x2E, 0x83, 0x3E, 0x03, 0x7C, 0x08, 0x74, 0x06, 0x2E, 0xC6,
                0x06, 0x64, 0x7D, 0x02, 0xBB, 0x00, 0x00, 0x2E, 0x8B, 0x0E, 0x03, 0x7C, 0x51, 0xB0,
                0x09, 0x2A, 0xC1, 0xB4, 0x00, 0x8B, 0xF0, 0x56, 0x33, 0xD2, 0x33, 0xC0, 0x8A, 0xC5,
                0x2E, 0xF6, 0x36, 0x64, 0x7D, 0x8A, 0xE8, 0x8A, 0xF4, 0x8B, 0xC6, 0xB4, 0x02, 0xCD,
                0x13, 0x72, 0x2D, 0x5E, 0x59, 0x2E, 0x29, 0x36, 0x05, 0x7C, 0x74, 0x1F, 0x8B, 0xC6,
                0x2E, 0xF7, 0x26, 0x65, 0x7D, 0x03, 0xD8, 0xFE, 0xC5, 0xB1, 0x01, 0x51, 0xBE, 0x08,
                0x00, 0x2E, 0x3B, 0x36, 0x05, 0x7C, 0x7C, 0x05, 0x2E, 0x8B, 0x36, 0x05, 0x7C, 0xEB,
                0xC0, 0xEA, 0x00, 0x00, 0x60, 0x00, 0xBE, 0x67, 0x7D, 0xE8, 0x02, 0x00, 0xEB, 0xFE,
                0x32, 0xFF, 0x2E, 0xAC, 0x24, 0x7F, 0x74, 0x0B, 0x56, 0xB4, 0x0E, 0xBB, 0x07, 0x00,
                0xCD, 0x10, 0x5E, 0xEB, 0xEF, 0xC3, 0xE9, 0x33, 0xFF, 0xBB, 0x00, 0x00, 0xB9, 0x04,
                0x00, 0xB8, 0x01, 0x02, 0xCD, 0x13, 0x1E, 0x72, 0x33, 0x8C, 0xC8, 0x8E, 0xD8, 0xBF,
                0x00, 0x00, 0xB9, 0x0B, 0x00, 0x26, 0x80, 0x0D, 0x20, 0x26, 0x80, 0x4D, 0x20, 0x20,
                0x47, 0xE2, 0xF4, 0xBF, 0x00, 0x00, 0xBE, 0x8B, 0x7D, 0xB9, 0x0B, 0x00, 0xFC, 0xF3,
                0xA6, 0x75, 0x0F, 0xBF, 0x20, 0x00, 0xBE, 0x97, 0x7D, 0xB9, 0x0B, 0x00, 0xF3, 0xA6,
                0x75, 0x02, 0x1F, 0xC3, 0xBE, 0x1B, 0x7D, 0xE8, 0xA2, 0xFF, 0xB4, 0x00, 0xCD, 0x16,
                0x1F, 0xF9, 0xC3, 0x0D, 0x0A, 0x4E, 0x6F, 0x6E, 0x2D, 0x53, 0x79, 0x73, 0x74, 0x65,
                0x6D, 0x20, 0x64, 0x69, 0x73, 0x6B, 0x20, 0x6F, 0x72, 0x20, 0x64, 0x69, 0x73, 0x6B,
                0x20, 0x65, 0x72, 0x72, 0x6F, 0x72, 0x0D, 0x0A, 0x52, 0x65, 0x70, 0x6C, 0x61, 0x63,
                0x65, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6B, 0x65, 0x20, 0x61,
                0x6E, 0x79, 0x20, 0x6B, 0x65, 0x79, 0x20, 0x77, 0x68, 0x65, 0x6E, 0x20, 0x72, 0x65,
                0x61, 0x64, 0x79, 0x0D, 0x0A, 0x00, 0x01, 0x00, 0x02, 0x0D, 0x0A, 0x44, 0x69, 0x73,
                0x6B, 0x20, 0x42, 0x6F, 0x6F, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6C, 0x75, 0x72, 0x65,
                0x0D, 0x0A, 0x00, 0x4D, 0x69, 0x63, 0x72, 0x6F, 0x73, 0x6F, 0x66, 0x74, 0x2C, 0x49,
                0x6E, 0x63, 0x20, 0x69, 0x62, 0x6D, 0x62, 0x69, 0x6F, 0x20, 0x20, 0x63, 0x6F, 0x6D,
                0x30, 0x69, 0x62, 0x6D, 0x64, 0x6F, 0x73, 0x20, 0x20, 0x63, 0x6F, 0x6D, 0x30, 0x05,
                0xC6, 0x06, 0x77, 0x2F, 0xFF, 0x83, 0x7E, 0xFC, 0x00, 0x75, 0x0B, 0x80, 0x7E, 0xF7,
                0x3B, 0x75, 0x05, 0xC6, 0x06, 0x76, 0x2F, 0xFF, 0x89, 0xEC, 0x5D, 0xCA, 0x04, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            ])),
            Self::IBMDOS200 => {
                let bpb = BPB::from_disktype(disktype)?;
                 Ok(Some(vec![
                0xEB, 0x2C, 0x90, 0x49, 0x42, 0x4D, 0x20, 0x20, 0x32, 0x2E, 0x30, 0x00, 0x02, 0x02,
                0x01, 0x00, 0x02, 0x40, 0x00, 0x68, 0x01, 0xFC, 0x02, 0x00, 0x09, 0x00, 0x01, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x0A, 0xDF, 0x02, 0x25, 0x02, 0x09, 0x2A, 0xFF, 0x50, 0xF6,
                0x00, 0x02, 0xCD, 0x19, 0xFA, 0x33, 0xC0, 0x8E, 0xD0, 0xBC, 0x00, 0x7C, 0x8E, 0xD8,
                0xA3, 0x7A, 0x00, 0xC7, 0x06, 0x78, 0x00, 0x21, 0x7C, 0xFB, 0xCD, 0x13, 0x73, 0x03,
                0xE9, 0x95, 0x00, 0x0E, 0x1F, 0xA0, 0x10, 0x7C, 0x98, 0xF7, 0x26, 0x16, 0x7C, 0x03,
                0x06, 0x1C, 0x7C, 0x03, 0x06, 0x0E, 0x7C, 0xA3, 0x03, 0x7C, 0xA3, 0x13, 0x7C, 0xB8,
                0x20, 0x00, 0xF7, 0x26, 0x11, 0x7C, 0x05, 0xFF, 0x01, 0xBB, 0x00, 0x02, 0xF7, 0xF3,
                0x01, 0x06, 0x13, 0x7C, 0xE8, 0x7E, 0x00, 0x72, 0xB3, 0xA1, 0x13, 0x7C, 0xA3, 0x7E,
                0x7D, 0xB8, 0x70, 0x00, 0x8E, 0xC0, 0x8E, 0xD8, 0xBB, 0x00, 0x00, 0x2E, 0xA1, 0x13,
                0x7C, 0xE8, 0xB6, 0x00, 0x2E, 0xA0, 0x18, 0x7C, 0x2E, 0x2A, 0x06, 0x15, 0x7C, 0xFE,
                0xC0, 0x32, 0xE4, 0x50, 0xB4, 0x02, 0xE8, 0xC1, 0x00, 0x58, 0x72, 0x38, 0x2E, 0x28,
                0x06, 0x20, 0x7C, 0x76, 0x0E, 0x2E, 0x01, 0x06, 0x13, 0x7C, 0x2E, 0xF7, 0x26, 0x0B,
                0x7C, 0x03, 0xD8, 0xEB, 0xCE, 0x0E, 0x1F, 0xCD, 0x11, 0xD0, 0xC0, 0xD0, 0xC0, 0x25,
                0x03, 0x00, 0x75, 0x01, 0x40, 0x40, 0x8B, 0xC8, 0xF6, 0x06, 0x1E, 0x7C, 0x80, 0x75,
                0x02, 0x33, 0xC0, 0x8B, 0x1E, 0x7E, 0x7D, 0xEA, 0x00, 0x00, 0x70, 0x00, 0xBE, 0xC9,
                0x7D, 0xE8, 0x02, 0x00, 0xEB, 0xFE, 0x2E, 0xAC, 0x24, 0x7F, 0x74, 0x4D, 0xB4, 0x0E,
                0xBB, 0x07, 0x00, 0xCD, 0x10, 0xEB, 0xF1, 0xB8, 0x50, 0x00, 0x8E, 0xC0, 0x0E, 0x1F,
                0x2E, 0xA1, 0x03, 0x7C, 0xE8, 0x43, 0x00, 0xBB, 0x00, 0x00, 0xB8, 0x01, 0x02, 0xE8,
                0x58, 0x00, 0x72, 0x2C, 0x33, 0xFF, 0xB9, 0x0B, 0x00, 0x26, 0x80, 0x0D, 0x20, 0x26,
                0x80, 0x4D, 0x20, 0x20, 0x47, 0xE2, 0xF4, 0x33, 0xFF, 0xBE, 0xDF, 0x7D, 0xB9, 0x0B,
                0x00, 0xFC, 0xF3, 0xA6, 0x75, 0x0E, 0xBF, 0x20, 0x00, 0xBE, 0xEB, 0x7D, 0xB9, 0x0B,
                0x00, 0xF3, 0xA6, 0x75, 0x01, 0xC3, 0xBE, 0x80, 0x7D, 0xE8, 0xA6, 0xFF, 0xB4, 0x00,
                0xCD, 0x16, 0xF9, 0xC3, 0x1E, 0x0E, 0x1F, 0x33, 0xD2, 0xF7, 0x36, 0x18, 0x7C, 0xFE,
                0xC2, 0x88, 0x16, 0x15, 0x7C, 0x33, 0xD2, 0xF7, 0x36, 0x1A, 0x7C, 0x88, 0x16, 0x1F,
                0x7C, 0xA3, 0x08, 0x7C, 0x1F, 0xC3, 0x2E, 0x8B, 0x16, 0x08, 0x7C, 0xB1, 0x06, 0xD2,
                0xE6, 0x2E, 0x0A, 0x36, 0x15, 0x7C, 0x8B, 0xCA, 0x86, 0xE9, 0x2E, 0x8B, 0x16, 0x1E,
                0x7C, 0xCD, 0x13, 0xC3, 0x00, 0x00, 0x0D, 0x0A, 0x4E, 0x6F, 0x6E, 0x2D, 0x53, 0x79,
                0x73, 0x74, 0x65, 0x6D, 0x20, 0x64, 0x69, 0x73, 0x6B, 0x20, 0x6F, 0x72, 0x20, 0x64,
                0x69, 0x73, 0x6B, 0x20, 0x65, 0x72, 0x72, 0x6F, 0x72, 0x0D, 0x0A, 0x52, 0x65, 0x70,
                0x6C, 0x61, 0x63, 0x65, 0x20, 0x61, 0x6E, 0x64, 0x20, 0x73, 0x74, 0x72, 0x69, 0x6B,
                0x65, 0x20, 0x61, 0x6E, 0x79, 0x20, 0x6B, 0x65, 0x79, 0x20, 0x77, 0x68, 0x65, 0x6E,
                0x20, 0x72, 0x65, 0x61, 0x64, 0x79, 0x0D, 0x0A, 0x00, 0x0D, 0x0A, 0x44, 0x69, 0x73,
                0x6B, 0x20, 0x42, 0x6F, 0x6F, 0x74, 0x20, 0x66, 0x61, 0x69, 0x6C, 0x75, 0x72, 0x65,
                0x0D, 0x0A, 0x00, 0x69, 0x62, 0x6D, 0x62, 0x69, 0x6F, 0x20, 0x20, 0x63, 0x6F, 0x6D,
                0x30, 0x69, 0x62, 0x6D, 0x64, 0x6F, 0x73, 0x20, 0x20, 0x63, 0x6F, 0x6D, 0x30, 0x00,
                0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x55, 0xAA,
            ]))},
            Self::NONE => Ok(None),
            Self::UNKNOWN => Ok(Some(vec![0u8; 512])),
        }
    }
}

#[cfg(test)]
mod tests {
    use disk::disktype::DiskType;

    use crate::OperatingSystem;

    #[test]
    fn bootsector_ibmdos100() {
        let os = OperatingSystem::from_str("IBMDOS100");
        let bootsector = os.get_bootsector(&DiskType::F525_160).unwrap().unwrap();
        assert_eq!(bootsector.len(), 512);
    }

    #[test]
    fn bootsector_ibmdos110() {
        let os = OperatingSystem::from_str("IBMDOS110");
        let bootsector = os.get_bootsector(&DiskType::F525_160).unwrap().unwrap();
        assert_eq!(bootsector.len(), 512);
    }
}
