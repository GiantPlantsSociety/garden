#[macro_use] extern crate structopt;
extern crate failure;
extern crate garden;

use std::process::exit;
use failure::Error;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
struct InfoArgs {
    name: String,
}

/// Command line interface for managing data dependencies.
///
/// Visit `documentation` to learn more.
#[derive(Debug, StructOpt)]
#[structopt(name = "garden")]
enum Args {
    /// Search for pots
    #[structopt(name = "search")]
    Search {
        pattern: String,
    },
    /// Display pot info
    #[structopt(name = "info")]
    Info(InfoArgs),
    /// Add new pot into system
    #[structopt(name = "add")]
    Add {
        #[structopt(long = "dry-run")]
        dry_run: bool,
        names: Vec<String>
    }
}

fn info_command(args: &InfoArgs) -> Result<(), Error> {
    use garden::svalbard::{SeedVault, greenhouse::GreenHouse};

    let vault = GreenHouse::new();
    match vault.lookup(&args.name)? {
        None => println!("No pots named '{}' found.", args.name),
        Some(pot) => println!("{:#?}", pot),
    }
    Ok(())
}

fn run(args: &Args) -> Result<(), Error> {
    match *args {
        Args::Info(ref args) => info_command(args),
        _ => unimplemented!()
    }
}

fn main() {
    let args = Args::from_args();
    if let Err(err) = run(&args) {
        eprintln!("{:#?}", err);
        exit(1);
    }
}
