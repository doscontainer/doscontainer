use error::PlanError;
use operatingsystem::{OperatingSystem, OsShortName};
use ossupport::{OsSupport, SUPPORTED_OS};
use specs::{
    hwspec::HwSpec,
    manifest::Manifest,
};

mod error;
mod ossupport;

#[derive(Debug)]
pub struct InstallationPlanner {
    manifest: Manifest,
    os: OperatingSystem,
}

impl InstallationPlanner {
    /// Pass an HwSpec and an OsSupport into this function to figure out
    /// if the requested OS will run on the provided hardware. This function
    /// filters out operating systems so that only a compatible set remains.
    fn is_compatible(hwspec: &HwSpec, os: &OsSupport) -> (bool, OsShortName) {
        let compat = hwspec.ram() >= os.min_ram_kib
            && os.supported_cpu_families.contains(&hwspec.cpu().family())
            && hwspec
                .floppy_type()
                .as_ref()
                .map_or(false, |f| os.supported_floppies.contains(f))
            && !hwspec.video().is_empty()
            && hwspec
                .video()
                .iter()
                .any(|v| os.supported_video.contains(v));
        (compat, os.shortname)
    }

    pub fn new(hwspec: &HwSpec, manifest: Manifest) -> Result<InstallationPlanner, PlanError> {
        let mut compatible_os = Vec::new();

        // Step 1: Filter SUPPORTED_OS by hardware compatibility
        for os in SUPPORTED_OS.iter() {
            if Self::is_compatible(hwspec, os).0 {
                compatible_os.push(os);
            }
        }

        // Step 2: Filter compatible_os against manifest layers
        compatible_os.retain(|os| {
            manifest.layers().iter().all(|layer| {
                let version_ok = layer.1.min_dos().map_or(true, |min| os.version >= min)
                    && layer.1.max_dos().map_or(true, |max| os.version <= max);

                //let vendor_ok = layer.1.dos_vendors().contains(&os.shortname.vendor());

                version_ok// && vendor_ok
            })
        });

        // Step 3: Select the most "optimal" OS
        let selected = compatible_os
            .into_iter()
            .max_by(|a, b| a.version.cmp(&b.version)) // or custom strategy
            .ok_or(PlanError::NoCompatibleOS)?;

        Ok(InstallationPlanner {
            manifest,
            os: OperatingSystem::from_osshortname(&selected.shortname),
        })
    }

    /*

        let mut disk = RawImage::new(Path::new("/home/bvdwiel/test.img"), SectorSize::S512, 320)
            .expect("Failed to create disk");
        disk.ibmwipe().unwrap();
        let mut volume = Volume::new(&mut disk, 0, 320);
        let os = OperatingSystem::from_vendor_version("IBM", "1.00").unwrap();
        let mut filesystem = Fat12::new(SectorSize::S512, 1, 312, &mut volume).unwrap();
        let date = NaiveDate::from_ymd_opt(1981, 8, 4)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();
        filesystem
            .mksysfile("IBMBIO.COM", os.iosys_bytes(), Some(date))
            .unwrap();
        filesystem
            .mksysfile("IBMDOS.COM", os.msdossys_bytes(), Some(date))
            .unwrap();
        filesystem
            .mkfile("COMMAND.COM", os.commandcom_bytes(), Some(date))
            .unwrap();

        // Do massively ugly hard-coded crud here!
        filesystem.write_crud();

        let layers = manifest.mut_layers();
        for layer in layers {
            println!("Downloading {}", layer.0);
            println!("{:?}", layer.1.download());
        }
        Ok(())
    } */
}
