use operatingsystem::{vendor::OsVendor, version::OsVersion, OsShortName};
use specs::types::{cpu::CpuFamily, storage::FloppyType, video::VideoDevice};

#[derive(Debug)]
pub struct OsSupport {
    pub shortname: OsShortName,
    pub vendor: OsVendor,
    pub version: OsVersion,
    pub min_ram_kib: u32,
    pub supported_cpu_families: &'static [CpuFamily],
    pub supported_floppies: &'static [FloppyType],
    pub supported_video: &'static [VideoDevice],
}

pub static SUPPORTED_OS: &[OsSupport] = &[
    OsSupport {
        shortname: OsShortName::IBMDOS100,
        vendor: OsVendor::IBM,
        version: OsVersion::new(1, 0),
        min_ram_kib: 64,
        supported_cpu_families: &[CpuFamily::I8088],
        supported_floppies: &[FloppyType::F525_160],
        supported_video: &[VideoDevice::CGA, VideoDevice::MDA, VideoDevice::HGC],
    },
    OsSupport {
        shortname: OsShortName::IBMDOS110,
        vendor: OsVendor::IBM,
        version: OsVersion::new(1, 10),
        min_ram_kib: 64,
        supported_cpu_families: &[CpuFamily::I8088],
        supported_floppies: &[FloppyType::F525_160, FloppyType::F525_180],
        supported_video: &[VideoDevice::CGA, VideoDevice::MDA, VideoDevice::HGC],
    },
    OsSupport {
        shortname: OsShortName::IBMDOS200,
        vendor: OsVendor::IBM,
        version: OsVersion::new(2, 0),
        min_ram_kib: 128,
        supported_cpu_families: &[CpuFamily::I8088, CpuFamily::I8086],
        supported_floppies: &[
            FloppyType::F525_160,
            FloppyType::F525_180,
            FloppyType::F525_360,
        ],
        supported_video: &[VideoDevice::CGA, VideoDevice::MDA, VideoDevice::HGC],
    },
];
