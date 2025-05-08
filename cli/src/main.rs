use std::path::PathBuf;

use clap::{Parser, Subcommand};

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
        /// Filename of the build manifest, this is the YAML file you wrote.
        name: PathBuf,
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
        Commands::Build { name: _ } => {
            // Construct a container from the manifest

            // Download the layer content from the manifest

            // Write the OS layer to the disk image

            // Write all the other layers to the disk image
            println!("Press key");
            let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
            Ok(())
        }
        Commands::BuildCollection { startdir: _ } => {
            println!("Placeholder for collection builder");
            let manifest = manifest::loader::Loader::from_dir(std::path::Path::new("/home/bvdwiel/container"));
            println!("{:?}", manifest);
            Ok(())
        }
    }
}
