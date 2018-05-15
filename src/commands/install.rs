use pots::pot::Garden;
use pots::pot::Dependency;
use pots::pot::PotName;
use commands::add;

use error::*;
use std::str::FromStr;
use std::fs;
use std::io::Read;
use std::path::PathBuf;
use toml;
use semver::VersionReq;

#[derive(Debug, StructOpt)]
pub struct Args {
}

// add
// - [*] update registry
// - check meta
// - [*] init garden
// - add to garden.toml
// - add to garden.lock
// - install exact version from lock
// - check exact version from lock

pub fn command(_args: &Args) -> Result<()> {
    let mut s = String::new();
    let filename = "garden.toml";
    let mut file = fs::File::open(&filename).map_err(|_| Error::FileNotFound(filename.to_string()))?;
    file.read_to_string(&mut s).map_err(Error::Io)?;
    let config: Garden = toml::from_str(&s).map_err(Error::TomlParseError)?;

    println!("Installing dependencies from '{}'", &filename);
    for (name, dependency) in &config.dependencies {
        match dependency {
            Dependency::Version(version) =>
                add::add(&add::Requirement::Name {
                    name: PotName::from_str(name).map_err(Error::LookupError)?,
                    version: VersionReq::parse(version).map_err(Error::VersionParseError)?,
                })?,
            Dependency::Reference(location) =>
                add::add(&add::Requirement::LocalPath(PathBuf::from(location.path.clone())))?,
        }
    }
    Ok(())
}
