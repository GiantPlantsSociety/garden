use toml;
use semver::VersionReq;
use error::Result;
use pots::pot::{Pot, PotName};
use super::Repository;

pub struct GreenHouse(Vec<Pot>);

impl GreenHouse {
    pub fn new() -> Self {
        let mut repo = GreenHouse(vec![]);
        repo.publish(&toml::from_str(include_str!("greenhouse/baby_names_0.2.0.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/baby_names_0.1.0.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/cifar_100.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/cifar_10.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/fashion_mnist.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/mnist.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/trump_tweets.toml")).unwrap()).unwrap();
        repo.publish(&toml::from_str(include_str!("greenhouse/uci_banking.toml")).unwrap()).unwrap();
        repo
    }
}

impl Repository for GreenHouse {
    fn lookup(&self, name: &PotName, version_req: &VersionReq) -> Result<Option<Pot>> {
        for pot in &self.0 {
            if pot.name == *name && version_req.matches(&pot.version) {
                return Ok(Some(pot.clone()));
            }
        }
        Ok(None)
    }

    fn search(&self, pattern: &str) -> Result<Vec<Pot>> {
        let mut result = Vec::new();
        for pot in &self.0 {
            if pot.name.to_string().contains(pattern) || pot.description.contains(pattern) {
                result.push(pot.clone());
            }
        }
        Ok(result)
    }

    fn publish(&mut self, pot: &Pot) -> Result<()> {
        self.0.push(pot.clone());
        Ok(())
    }
}
