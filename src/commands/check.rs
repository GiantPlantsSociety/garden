use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use pots::pot::Pot;
use process::*;
use summator::{Summator, Sums};
use error::*;

use std::fs;
use std::path::Path;

use url::Url;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub names: Vec<String>,
}

fn check_pot_files(pot: &Pot) -> Result<()> {
    for file in &pot.files {
        let ref url = file.url;
        let mut u = Url::parse(url).map_err(Error::Parse)?;
        let filename = u.path_segments().unwrap().last().unwrap();

        let base = Path::new("garden_data").join(&pot.name);
        let path = base.join(&filename);

        let metadata = fs::metadata(&path).map_err(|_| Error::FileNotFound(path.display().to_string()))?;
        let bytes_total = metadata.len();

        println!("   Checking: {}", path.display());

        let mut inp = fs::File::open(&path).map_err(Error::Io)?;
        let mut out = Summator::default();
        process_bytes_and_display_progress(&mut inp, &mut out, bytes_total)?;

        let checksums: Sums = out.into();
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
                check_pot_files(&pot)?;
            }
        }
    }
    Ok(())
}
