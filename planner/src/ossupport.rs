use operatingsystem::OsShortName;
use specs::types::{cpu::CpuFamily, storage::FloppyType, video::VideoDevice};

pub struct OsSupport {
    pub version: OsShortName,
    pub min_ram_kib: u32,
    pub supported_cpu_families: &'static [CpuFamily],
    pub supported_floppies: &'static [FloppyType],
    pub supported_video: &'static [VideoDevice],
}

pub static SUPPORTED_OS: &[OsSupport] = &[
    OsSupport {
        version: OsShortName::IBMDOS100,
        min_ram_kib: 64,
        supported_cpu_families: &[CpuFamily::I8088],
        supported_floppies: &[FloppyType::F525_160],
        supported_video: &[VideoDevice::CGA, VideoDevice::MDA, VideoDevice::HGC],
    },
    OsSupport {
        version: OsShortName::IBMDOS110,
        min_ram_kib: 64,
        supported_cpu_families: &[CpuFamily::I8088],
        supported_floppies: &[FloppyType::F525_160, FloppyType::F525_180],
        supported_video: &[VideoDevice::CGA, VideoDevice::MDA, VideoDevice::HGC],
    },
    OsSupport {
        version: OsShortName::IBMDOS200,
        min_ram_kib: 128,
        supported_cpu_families: &[CpuFamily::I8088, CpuFamily::I8086],
        supported_floppies: &[
            FloppyType::F525_160,
            FloppyType::F525_180,
            FloppyType::F525_360,
        ],
        supported_video: &[VideoDevice::CGA, VideoDevice::MDA, VideoDevice::HGC]
    },
];
