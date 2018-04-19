pub mod greenhouse;

use error::Result;
use pots::pot::Pot;

pub trait SeedVault {
    fn lookup(&self, name: &str) -> Result<Option<Pot>>;
    fn search(&self, pattern: &str) -> Result<Vec<Pot>>;
}
