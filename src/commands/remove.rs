use svalbard::greenhouse::GreenHouse;
use svalbard::Repository;
use pots::{Pot, PotName, Garden};
use error::*;

use url::Url;
use semver::VersionReq;
use dialoguer::Confirmation;

use std::fs;

#[derive(Debug, StructOpt)]
pub struct Args {
    pub name: PotName,
}

fn remove_pot_files(garden: &Garden, pot: &Pot) -> Result<()> {
    let base = garden.pot_location(pot);

    for file in &pot.files {
        let ref url = file.url;
        let mut u = Url::parse(url).map_err(Error::Parse)?;
        let filename = u.path_segments().unwrap().last().unwrap();
        let path = base.join(&filename);

        println!("   Deleting: {}", path.display());
        fs::remove_file(&path).map_err(|_| Error::FileNotFound(path.display().to_string()))?;
    }

    println!("   Deleting: {}", base.display());
    fs::remove_dir(&base)
        .or_else(|_| {
            if Confirmation::new("Directory still contains some files. Do you really want to delete it?")
                .default(false)
                .use_line_input(false)
                .interact()?
            {
                fs::remove_dir_all(&base)
            } else {
                println!("Not all the files were removed from '{}'", &base.display());
                Ok(())
            }
        })
        .map_err(Error::Io)?;
    Ok(())
}

pub fn command(args: &Args) -> Result<()> {
    let garden = Garden::new(".")?;

    let repo = GreenHouse::new();
    if let Some(pot) = repo.lookup(&args.name, &VersionReq::any())? {
        println!("   Removing: {}", pot.name);
        remove_pot_files(&garden, &pot)?;
    } else {
        return Err(Error::LookupError(args.name.to_string()));
    }
    Ok(())
}
