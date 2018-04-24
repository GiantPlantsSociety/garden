use error::*;
use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;

#[derive(Debug, StructOpt)]
pub struct Args {
    pattern: String,
}

pub fn command(args: &Args) -> Result<()> {
    let repo = GreenHouse::new();
    let pots = repo.search(&args.pattern)?;
    if pots.is_empty() {
        println!("No pots matching pattern '{}' found.", args.pattern);
    } else {
        println!("{:#?}", pots);
    }
    Ok(())
}
