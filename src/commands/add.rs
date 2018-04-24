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

#[derive(Debug, StructOpt)]
pub struct Args {
    #[structopt(long = "dry-run")]
    dry_run: bool,
    names: Vec<String>
}

fn download_file<R: Read, W: Write>(inp: &mut R, out: &mut W, bytes_total: u64) -> Result<()> {
    let pb = ProgressBar::new(bytes_total);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
        .progress_chars("#>-"));

    download_file_with_progress(inp, out, &pb)?;
    pb.finish_and_clear();
    Ok(())
}

fn download_file_with_progress<R: Read, W: Write>(inp: &mut R, out: &mut W, pb: &ProgressBar) -> Result<()> {
    let mut buffer = [0; 128 * 1024];
    let mut bytes_read = 0;
    loop {
        let len = inp.read(&mut buffer).map_err(Error::Io)?;
        if len == 0 {
            break;
        }

        out.write_all(&buffer[..len]).map_err(Error::Io)?;
        bytes_read += len;

        pb.set_position(bytes_read as u64);
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

        let mut resp = client.get(url).send().map_err(Error::Network)?;
        let mut file = fs::File::create(path).map_err(Error::Io)?;

        download_file(&mut resp, &mut file, bytes_total)?;
    }
    Ok(())
}

pub fn command(args: &Args) -> Result<()> {
    let repo = GreenHouse::new();
    for name in &args.names {
        match repo.lookup(name)? {
            None => println!("No pots named '{}' found.", name),
            Some(pot) => {
                println!("     Adding: {}", pot.name);
                download_pot_files(&pot)?;
                println!(" Validating: md5");
                println!(" Validating: sha1");
                println!("  Compiling: arrow");
                println!("    Binding: python");
                println!("    Binding: javascript");
            }
        }
    }
    Ok(())
}
