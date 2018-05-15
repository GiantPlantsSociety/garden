pub mod greenhouse;
pub mod local_data;

use semver::VersionReq;
use error::*;
use pots::pot::{Pot, PotName};

pub trait Repository {
    fn lookup(&self, name: &PotName, version_req: &VersionReq) -> Result<Option<Pot>>;
    fn search(&self, pattern: &str) -> Result<Vec<Pot>>;
    fn publish(&mut self, pot: &Pot) -> Result<()>;

    fn lookup_or_suggest(&self, name: &PotName, version_req: &VersionReq) -> Result<Pot> {
        if let Some(pot) = self.lookup(name, version_req)? {
            return Ok(pot);
        }
        if let Some(pot) = self.lookup(name, &VersionReq::parse("*").unwrap())? {
            return Err(Error::LookupErrorWithVersionSuggestion(name.to_string(), pot.version.to_string()));
        }
        Err(Error::LookupError(name.to_string()))
    }
}

pub trait Workspace {
    fn exists(&self, name: &str) -> Result<Option<String>>;
}
