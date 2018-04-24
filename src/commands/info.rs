use error::*;
use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;

#[derive(Debug, StructOpt)]
pub struct Args {
    name: String,
}

pub fn command(args: &Args) -> Result<()> {
    let repo = GreenHouse::new();
    match repo.lookup(&args.name)? {
        None => println!("No pots named '{}' found.", args.name),
        Some(pot) => println!("{:#?}", pot),
    }
    Ok(())
}
