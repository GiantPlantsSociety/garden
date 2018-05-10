use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use pots::pot::Pot;
use commands::check;
use process::*;

use reqwest::Client;
use reqwest::header::ContentLength;
use url::Url;

use error::*;
use semver::VersionReq;
use std::fs;
use std::path::Path;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub name: String,
    #[structopt(default_value = "*")]
    pub version: VersionReq,
}

fn download_pot_files(pot: &Pot) -> Result<()> {
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

        let base = Path::new("garden_data").join(&pot.name);
        fs::create_dir_all(&base).map_err(Error::Io)?;

        let path = base.join(&filename);
        println!("Downloading: {} => {}", url, path.display());

        let mut response = client.get(url).send().map_err(Error::Network)?;
        let mut output = fs::File::create(path.clone()).map_err(Error::Io)?;

        process_bytes_and_display_progress(&mut response, &mut output, bytes_total)?;
    }
    Ok(())
}

pub fn command(args: &Args) -> Result<()> {
    let repo = GreenHouse::new();

    let pot = repo.lookup_or_suggest(&args.name, &args.version)?;

    println!();
    println!("  🌱  Adding: {} v{}", pot.name, pot.version);
    download_pot_files(&pot)?;
    check::command(&check::Args {
        name: args.name.clone(),
        version: args.version.clone(),
    })?;
    println!("  Compiling: arrow");
    println!("    Binding: python");
    println!("    Binding: javascript");

    Ok(())
}
