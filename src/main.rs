#[macro_use] extern crate structopt;
extern crate failure;
extern crate garden;

use garden::error::*;
use std::process::exit;
use structopt::StructOpt;

use garden::commands::{
    info,
    search,
    add,
    install,
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
    /// Install dependencies specified in garden.toml config
    #[structopt(name = "install")]
    Install(install::Args),
}

fn run(args: &Args) -> Result<()> {
    match *args {
        Args::Search(ref args) => search::command(args),
        Args::Info(ref args) => info::command(args),
        Args::Add(ref args) => add::command(args),
        Args::Install(ref args) => install::command(args),
        // _ => unimplemented!()
    }
}

fn main() {
    let args = Args::from_args();
    if let Err(err) = run(&args) {
        eprintln!("{}", err);
        exit(1);
    }
}
