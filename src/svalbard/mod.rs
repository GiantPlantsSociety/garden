pub mod greenhouse;
pub mod local_data;

use error::Result;
use pots::pot::Pot;

pub trait Repository {
    fn lookup(&self, name: &str) -> Result<Option<Pot>>;
    fn search(&self, pattern: &str) -> Result<Vec<Pot>>;
    fn publish(&mut self, pot: &Pot) -> Result<()>;
}

pub trait Workspace {
    fn exists(&self, name: &str) -> Result<Option<String>>;
}
