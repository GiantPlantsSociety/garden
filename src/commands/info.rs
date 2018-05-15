use semver::VersionReq;
use error::*;
use pots::pot::PotName;
use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub name: PotName,
    #[structopt(default_value = "*")]
    pub version: VersionReq,
}

pub fn command(args: &Args) -> Result<()> {
    let repo = GreenHouse::new();
    let pot = repo.lookup_or_suggest(&args.name, &args.version)?;
    println!("{:#?}", pot);
    Ok(())
}
