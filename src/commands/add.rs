use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use pots::{Pot, PotName, Garden};
use commands::check;
use process::*;

use reqwest::Client;
use reqwest::header::ContentLength;
use url::Url;
use toml;

use error::*;
use semver::VersionReq;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

#[derive(Debug)]
pub enum Requirement {
    Name { name: PotName, version: VersionReq, },
    LocalPath(PathBuf),
}

fn parse_requirement_with_version(s: &str) -> Option<(PotName, VersionReq)> {
    let parts: Vec<_> = s.splitn(2, '@').collect();
    let name = PotName::from_str(&parts.get(0)?).ok()?;
    let version = VersionReq::from_str(&parts.get(1)?).ok()?;
    Some((name, version))
}

impl FromStr for Requirement {
    type Err = String;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        if let Ok(name) = PotName::from_str(s) {
            return Ok(Requirement::Name { name, version: VersionReq::any() });
        }
        if let Some((name, version)) = parse_requirement_with_version(s) {
            return Ok(Requirement::Name { name, version });
        }
        if Path::new(s).is_file() {
            return Ok(Requirement::LocalPath(Path::new(s).to_path_buf()));
        }
        Err(format!("Bad pot requirement '{}'.", s))
    }
}

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(raw(required = "true", min_values = "1"))]
    pub pots: Vec<Requirement>,
}

fn download_pot_files(garden: &Garden, pot: &Pot) -> Result<()> {
    let base = garden.pot_location(pot);

    let client = Client::new();
    for file in &pot.files {
        let ref url = file.url;
        let mut u = Url::parse(url).map_err(Error::Parse)?;
        let filename = u.path_segments().unwrap().last().unwrap();

        let head = client.head(url).send().map_err(Error::Network)?;
        if !head.status().is_success() {
            return Err(Error::FileNotFound(url.to_string()));
        }

        let bytes_total = head.headers().get::<ContentLength>()
            .map(|ct_len| **ct_len)
            .unwrap_or(0);

        fs::create_dir_all(&base).map_err(Error::Io)?;

        let path = base.join(&filename);
        println!("Downloading: {} => {}", url, path.display());

        let mut response = client.get(url).send().map_err(Error::Network)?;
        let mut output = fs::File::create(path.clone()).map_err(Error::Io)?;

        process_bytes_and_display_progress(&mut response, &mut output, bytes_total)?;
    }
    Ok(())
}

pub fn add(garden: &Garden, requirement: &Requirement) -> Result<()> {
    let pot = match requirement {
        Requirement::Name { name, version } => {
            let repo = GreenHouse::new();
            repo.lookup_or_suggest(name, version)?
        },
        Requirement::LocalPath(path) => {
            let description = fs::read_to_string(path).map_err(Error::Io)?;
            toml::from_str(&description).map_err(Error::TomlParseError)?
        },
    };

    println!();
    println!("  🌱  Adding: {} v{}", pot.name, pot.version);
    download_pot_files(garden, &pot)?;
    check::command(&check::Args {
        name: pot.name.clone(),
        version: VersionReq::exact(&pot.version),
    })?;
    println!("  Compiling: arrow");
    println!("    Binding: python");
    println!("    Binding: javascript");

    Ok(())
}

pub fn command(args: &Args) -> Result<()> {
    let garden = Garden::new(".")?;
    for requirement in &args.pots {
        add(&garden, requirement)?;
    }
    Ok(())
}
