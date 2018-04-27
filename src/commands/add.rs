use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use pots::pot::Pot;

use reqwest::header::ContentLength;
use url::Url;

use error::*;
use std::fs;
use std::io::{Read, Write};
use std::path::Path;

use summator;

#[derive(Debug, StructOpt)]
pub struct Args {
    names: Vec<String>,
}

impl Args {
    pub fn new(names: Vec<String>) -> Self {
        Self { names }
    }
}

fn download_file<R: Read, W: Write>(inp: &mut R, out: &mut W, bytes_total: u64) -> Result<()> {
    let pb = ProgressBar::new(bytes_total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    process_file(inp, out, Some(&pb))?;
    pb.finish_and_clear();
    Ok(())
}

fn calculate_checksums(filename: &Path) -> Result<summator::Sums> {
    let mut f = fs::File::open(filename).map_err(Error::Io)?;
    let mut s = summator::Summator::default();
    process_file(&mut f, &mut s, None)?;
    Ok(s.into())
}

fn process_file<R: Read, W: Write>(inp: &mut R, out: &mut W, pb: Option<&ProgressBar>) -> Result<()> {
    let mut buffer = [0; 128 * 1024];
    let mut bytes_read = 0;
    loop {
        let len = inp.read(&mut buffer).map_err(Error::Io)?;
        if len == 0 {
            break;
        }

        out.write_all(&buffer[..len]).map_err(Error::Io)?;
        bytes_read += len;

        pb.map(|pb| pb.set_position(bytes_read as u64));
    }
    Ok(())
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

        download_file(&mut response, &mut output, bytes_total)?;

        let checksums = calculate_checksums(&path)?;

        if let Some(ref digest) = file.md5 {
            println!(" Validating: md5 => {}", &digest);
            if checksums.md5 != *digest {
                return Err(Error::FileChecksum(path.display().to_string()));
            }
        }

        if let Some(ref digest) = file.sha1 {
            println!(" Validating: sha1 => {}", &digest);
            if checksums.sha1 != *digest {
                return Err(Error::FileChecksum(path.display().to_string()));
            }
        }

        if let Some(ref digest) = file.sha2_256 {
            println!(" Validating: sha2_256 => {}", &digest);
            if checksums.sha2_256 != *digest {
                return Err(Error::FileChecksum(path.display().to_string()));
            }
        }

        if let Some(ref digest) = file.sha3_224 {
            println!(" Validating: sha3_224 => {}", &digest);
            if checksums.sha3_224 != *digest {
                return Err(Error::FileChecksum(path.display().to_string()));
            }
        }

        if let Some(ref digest) = file.sha3_256 {
            println!(" Validating: sha3_256 => {}", &digest);
            if checksums.sha3_256 != *digest {
                return Err(Error::FileChecksum(path.display().to_string()));
            }
        }
    }
    Ok(())
}

pub fn command(args: &Args) -> Result<()> {
    let repo = GreenHouse::new();
    for name in &args.names {
        match repo.lookup(name)? {
            None => println!("No pots named '{}' found.", name),
            Some(pot) => {
                println!();
                println!("  🌱  Adding: {}", pot.name);
                download_pot_files(&pot)?;
                println!("  Compiling: arrow");
                println!("    Binding: python");
                println!("    Binding: javascript");
            }
        }
    }
    Ok(())
}
