#[macro_use] extern crate structopt;
extern crate failure;

use std::process::exit;
use failure::Error;
use structopt::StructOpt;

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
    Info {
        name: String,
    },
    /// Add new pot into system
    #[structopt(name = "add")]
    Add {
        #[structopt(long = "dry-run")]
        dry_run: bool,
        names: Vec<String>
    }
}

fn run(args: &Args) -> Result<(), Error> {
    println!("{:?}", &args);
    Ok(())
}

fn main() {
    let args = Args::from_args();
    if let Err(err) = run(&args) {
        eprintln!("{:#?}", err);
        exit(1);
    }
}
