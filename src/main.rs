#[macro_use] extern crate structopt;
extern crate failure;
extern crate garden;

use std::process::exit;
use failure::Error;
use structopt::StructOpt;

use garden::commands::{
    info,
    search,
    add,
};

/// Command line interface for managing data dependencies.
///
/// Visit `documentation` to learn more.
#[derive(Debug, StructOpt)]
#[structopt(name = "garden")]
enum Args {
    /// Search for pots
    #[structopt(name = "search")]
    Search(search::Args),
    /// Display pot info
    #[structopt(name = "info")]
    Info(info::Args),
    /// Add new pot into system
    #[structopt(name = "add")]
    Add(add::Args),
}

fn run(args: &Args) -> Result<(), Error> {
    match *args {
        Args::Search(ref args) => search::command(args),
        Args::Info(ref args) => info::command(args),
        Args::Add(ref args) => add::command(args),
        // _ => unimplemented!()
    }
}

fn main() {
    let args = Args::from_args();
    if let Err(err) = run(&args) {
        eprintln!("{:#?}", err);
        exit(1);
    }
}
