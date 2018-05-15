use std::convert::AsRef;
use std::path::{Path, PathBuf};
use std::fs;
use toml;
use super::{GardenPlan, Pot};
use error::*;

pub struct Garden {
    path: PathBuf,
    pub plan: GardenPlan,
}

impl Garden {
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
