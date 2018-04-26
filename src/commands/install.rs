use pots::pot::Garden;

use error::*;
use std::fs;
use std::io::Read;
use toml;

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "dry-run")]
    dry_run: bool,
}

pub fn command(_args: &Args) -> Result<()> {
    let mut s = String::new();
    let mut file = fs::File::open("garden.toml").map_err(Error::Io)?;
    file.read_to_string(&mut s).map_err(Error::Io)?;
    let config: Garden = toml::from_str(&s).map_err(Error::TomlParseError)?;
    
    for dep in &config.dependencies {
        println!("{:?}", dep);
    }
    Ok(())
}
