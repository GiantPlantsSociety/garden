use pots::{Garden, Dependency, PotName};
use commands::add;

use error::*;
use std::str::FromStr;
use std::path::PathBuf;
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
    let garden = Garden::new(".")?;

    println!("Installing dependencies from garden.toml");
    for (name, dependency) in &garden.plan.dependencies {
        match dependency {
            Dependency::Version(version) =>
                add::add(&garden, &add::Requirement::Name {
                    name: PotName::from_str(name).map_err(Error::LookupError)?,
                    version: VersionReq::parse(version).map_err(Error::VersionParseError)?,
                })?,
            Dependency::Reference(location) =>
                add::add(&garden, &add::Requirement::LocalPath(PathBuf::from(location.path.clone())))?,
        }
    }
    Ok(())
}
