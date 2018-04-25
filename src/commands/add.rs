use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use reqwest::Client;
use indicatif::{ProgressBar, ProgressStyle};
use pots::pot::Pot;

use reqwest::header::ContentLength;
use url::Url;
use md5;
use sha1;
use sha2::Sha256;
use sha3::{Digest, Sha3_224, Sha3_256};
use hexx::{Hex16, Hex20, Hex28, Hex32};

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

fn hex_md5(filename: &Path) -> Result<Hex16> {
    let mut f = fs::File::open(filename).map_err(Error::Io)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(Error::Io)?;
    let digest = md5::compute(buffer);
    Ok(Hex16(digest.0))
}

fn hex_sha1(filename: &Path) -> Result<Hex20> {
    let mut f = fs::File::open(filename).map_err(Error::Io)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(Error::Io)?;
    let digest = sha1::Sha1::from(buffer).digest();
    Ok(Hex20(digest.bytes()))
}

fn hex_sha2_256(filename: &Path) -> Result<Hex32> {
    let mut f = fs::File::open(filename).map_err(Error::Io)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(Error::Io)?;
    let mut digest = Sha256::default();
    digest.input(&buffer);
    let mut a: [u8; 32] = Default::default();
    a.copy_from_slice(digest.result().as_slice());
    Ok(Hex32(a))
}

fn hex_sha3_224(filename: &Path) -> Result<Hex28> {
    let mut f = fs::File::open(filename).map_err(Error::Io)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(Error::Io)?;
    let mut digest = Sha3_224::default();
    digest.input(&buffer);
    let mut a: [u8; 28] = Default::default();
    a.copy_from_slice(digest.result().as_slice());
    Ok(Hex28(a))
}

fn hex_sha3_256(filename: &Path) -> Result<Hex32> {
    let mut f = fs::File::open(filename).map_err(Error::Io)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(Error::Io)?;
    let mut digest = Sha3_256::default();
    digest.input(&buffer);
    let mut a: [u8; 32] = Default::default();
    a.copy_from_slice(digest.result().as_slice());
    Ok(Hex32(a))
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

        let mut response = client.get(url).send().map_err(Error::Network)?;
        let mut output = fs::File::create(path.clone()).map_err(Error::Io)?;

        download_file(&mut response, &mut output, bytes_total)?;

        match file.md5 {
            Some(ref digest) => {
                println!(" Validating: md5 => {}", &digest);
                if hex_md5(&path)? != *digest {
                    return Err(Error::FileChecksum(path.display().to_string()));
                }
            },
            None => {},
        }

        match file.sha1 {
            Some(ref digest) => {
                println!(" Validating: sha1 => {}", &digest);
                if hex_sha1(&path)? != *digest {
                    return Err(Error::FileChecksum(path.display().to_string()));
                }
            },
            None => {},
        }

        match file.sha2_256 {
            Some(ref digest) => {
                println!(" Validating: sha2_256 => {}", &digest);
                if hex_sha2_256(&path)? != *digest {
                    return Err(Error::FileChecksum(path.display().to_string()));
                }
            },
            None => {},
        }

        match file.sha3_224 {
            Some(ref digest) => {
                println!(" Validating: sha3_224 => {}", &digest);
                if hex_sha3_224(&path)? != *digest {
                    return Err(Error::FileChecksum(path.display().to_string()));
                }
            },
            None => {},
        }

        match file.sha3_256 {
            Some(ref digest) => {
                println!(" Validating: sha3_256 => {}", &digest);
                if hex_sha3_256(&path)? != *digest {
                    return Err(Error::FileChecksum(path.display().to_string()));
                }
            },
            None => {},
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
                println!("     Adding: {}", pot.name);
                download_pot_files(&pot)?;
                println!("  Compiling: arrow");
                println!("    Binding: python");
                println!("    Binding: javascript");
            }
        }
    }
    Ok(())
}
