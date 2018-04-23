use failure::Error;
use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "dry-run")]
    dry_run: bool,
    names: Vec<String>
}

pub fn command(args: &Args) -> Result<(), Error> {
    let repo = GreenHouse::new();
    for name in &args.names {
        match repo.lookup(name)? {
            None => println!("No pots named '{}' found.", name),
            Some(pot) => println!("Adding '{}'", pot.name),
        }
    }
    Ok(())
}
