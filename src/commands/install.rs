use pots::pot::Garden;
use commands::add;

use error::*;
use std::fs;
use std::io::Read;
use toml;

#[derive(Debug, StructOpt)]
pub struct Args {
}

pub fn command(_args: &Args) -> Result<()> {
    let mut s = String::new();
    let filename = "garden.toml";
    let mut file = fs::File::open(&filename).map_err(|_| Error::FileNotFound(filename.to_string()))?;
    file.read_to_string(&mut s).map_err(Error::Io)?;
    let config: Garden = toml::from_str(&s).map_err(Error::TomlParseError)?;

    println!("Installing dependencies from '{}'", &filename);
    let len = config.dependencies.len();
    let mut i = 1;
    for (name, version) in &config.dependencies {
        println!();
        println!("[{}/{}] {}=\"{}\"", i, len, name, version);
        add::command(&add::Args {
            name: name.to_string(),
            version: version.clone(),
        })?;
        i += 1;
    }
    Ok(())
}
