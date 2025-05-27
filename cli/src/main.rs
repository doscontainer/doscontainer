use builder::Builder;
use clap::{Parser, Subcommand};
use specs::{hwspec::HwSpec, manifest::Manifest};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version="v0.0.1 'Smeagol'", author="Bas v.d. Wiel <bas@area536.com>", about="DOSContainer CLI utility", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Build a DOSContainer manifest into a disk image.
    Build {
        /// Image filename
        imagepath: PathBuf,
        /// Filename of the HwSpec configuration file, this a TOML file.
        hwspecpath: PathBuf,
        /// Filename of the build manifest.
        manifestpath: PathBuf,
    },
    /// Build a Collection of DOSContainer manifests into a library of disk images.
    BuildCollection { startdir: PathBuf },
    /// Download all dependencies required to host your own DOSContainer collections.
    SelfHost {
        /// Directory on your local machine that'll hold DOSContainer assets.
        docroot: PathBuf,
    },
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::SelfHost { docroot: _ } => {
            // Do nothing for now, just enable the command
            println!("[TODO] This still needs implementation.");
            Ok(())
        }
        Commands::Build {
            imagepath,
            hwspecpath,
            manifestpath,
        } => {
            // Construct a HW Spec from a TOML file
            let spec = HwSpec::from_toml(hwspecpath).expect("Failed loading HwSpec.");

            // Download the layer content from the manifest
            let manifest = Manifest::from_toml(manifestpath).expect("Failed loading Manifest.");

            // Invoke the Disk Image Planner
            let planner = planner::InstallationPlanner::new(spec, manifest).unwrap();
            let builder = Builder::new(planner);
            builder.build(&imagepath);
            Ok(())
        }
        Commands::BuildCollection { startdir: _ } => {
            println!("Placeholder for collection builder");
            Ok(())
        }
    }
}
