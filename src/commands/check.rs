use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use pots::{Pot, PotName, Garden};
use process::*;
use summator::{Summator, Sums};

use error::*;
use std::fs;
use semver::VersionReq;
use url::Url;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub name: PotName,
    #[structopt(default_value = "*")]
    pub version: VersionReq,
}

pub fn check_pot_files(garden: &Garden, pot: &Pot) -> Result<()> {
    let base = garden.pot_location(pot);

    for file in &pot.files {
        let ref url = file.url;
        let mut u = Url::parse(url).map_err(Error::Parse)?;
        let filename = u.path_segments().unwrap().last().unwrap();

        let path = base.join(&filename);

        if !base.exists() {
            return Err(Error::DirNotFound(base.display().to_string()));
        }

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
    let garden = Garden::new(".")?;

    let repo = GreenHouse::new();
    let pot = repo.lookup_or_suggest(&args.name, &args.version)?;
    check_pot_files(&garden, &pot)?;
    Ok(())
}
