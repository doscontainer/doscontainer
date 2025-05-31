use common::storage::Floppy;
use error::PlanError;
use operatingsystem::OperatingSystem;
use ossupport::{OsSupport, SUPPORTED_OS};
use specs::{hwspec::HwSpec, manifest::Manifest};

mod error;
mod ossupport;

#[derive(Debug)]
pub struct InstallationPlanner {
    hwspec: HwSpec,
    manifest: Manifest,
    os: OperatingSystem,
    floppy: Option<Floppy>,
}

impl InstallationPlanner {
    pub fn hwspec(&self) -> &HwSpec {
        &self.hwspec
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn os(&self) -> &OperatingSystem {
        &self.os
    }

    /// Determine if a specific OS is compatible with given hardware
    fn is_compatible(hwspec: &HwSpec, os: &OsSupport) -> bool {
        hwspec.ram() >= os.min_ram_kib
            && os.supported_cpu_families.contains(&hwspec.cpu().family())
            && hwspec
                .floppy_type()
                .as_ref()
                .map_or(false, |f| os.supported_floppies.contains(f))
            && !hwspec.video().is_empty()
            && hwspec
                .video()
                .iter()
                .any(|v| os.supported_video.contains(v))
    }

    pub fn new(hwspec: HwSpec, manifest: Manifest) -> Result<InstallationPlanner, PlanError> {
        // Step 1: Filter SUPPORTED_OS by hardware compatibility
        let mut compatible_os: Vec<&OsSupport> = SUPPORTED_OS
            .iter()
            .filter(|os| Self::is_compatible(&hwspec, os))
            .collect();

        // Step 2: Keep only OSes that are compatible with *all* layers
        compatible_os.retain(|os| {
            manifest.layers().iter().all(|layer| {
                let version_ok = layer.1.min_dos().map_or(true, |min| os.version >= min)
                    && layer.1.max_dos().map_or(true, |max| os.version <= max);

                let vendors = layer.1.dos_vendors();
                let vendor_ok = vendors.is_empty() || vendors.contains(&os.shortname.vendor());

                version_ok && vendor_ok
            })
        });

        if compatible_os.is_empty() {
            return Err(PlanError::NoCompatibleOS);
        }

        // Step 3: Select the highest version among compatible OSes
        let max_version = compatible_os.iter().map(|os| os.version).max().unwrap(); // safe: list not empty

        let best_os_candidates: Vec<&OsSupport> = compatible_os
            .into_iter()
            .filter(|os| os.version == max_version)
            .collect();

        // Pick the first (deterministically) — or change logic to return all
        let selected = best_os_candidates[0];

        Ok(InstallationPlanner {
            floppy: hwspec.floppy_type(),
            hwspec,
            manifest,
            os: OperatingSystem::from_osshortname(&selected.shortname),
        })
    }
}
