use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fs;
use std::collections::HashMap;
use std::io::Write;
use toml;
use super::{GardenPlan, Pot};
use error::*;

pub struct Garden {
    path: PathBuf,
    pub plan: GardenPlan,
}

impl Garden {
    pub fn init(base: impl AsRef<Path>) -> Result<()> {
        let path = base.as_ref().join("garden.toml");
        if !path.exists() {
            println!("Creating empty 'garden.toml' file.");
            let plan = GardenPlan {
                dependencies: HashMap::new(),
            };
            let toml = toml::to_string(&plan).map_err(Error::TomlWriteError)?;
            let mut file = fs::File::create(path).map_err(Error::Io)?;
            file.write_all(toml.as_bytes()).map_err(Error::Io)?;
        } else {
            println!("Nothing to do: 'garden.toml' file is already present.");
        }
        Ok(())
    }

    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let plan_bytes = fs::read(path.as_ref().join("garden.toml")).map_err(Error::Io)?;
        let plan = toml::from_slice(&plan_bytes).map_err(Error::TomlParseError)?;
        Ok(Self {
            path: path.as_ref().to_path_buf(),
            plan,
        })
    }

    pub fn pot_location(&self, pot: &Pot) -> PathBuf {
        self.path.join("garden_data").join(&pot.name.to_string())
    }
}
