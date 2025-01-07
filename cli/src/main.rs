use std::path::PathBuf;

use clap::{Parser, Subcommand};
use core::DosContainer;

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
}

fn main() -> Result<(), std::io::Error> {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Build { name } => {
            let mut container = DosContainer::new(name).unwrap();
            container.download_layers().expect("BLAM!");
            println!("Press key");
            let _ = std::io::stdin().read_line(&mut String::new()).unwrap();
            Ok(())
        }
    }
}
