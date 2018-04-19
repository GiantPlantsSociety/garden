use toml;
use error::Result;
use pots::pot::Pot;
use super::SeedVault;

pub struct GreenHouse(Vec<Pot>);

impl GreenHouse {
    pub fn new() -> Self {
        GreenHouse(vec![
            toml::from_str(include_str!("../greenhouse/baby_names.toml")).unwrap(),
            toml::from_str(include_str!("../greenhouse/cifar_100.toml")).unwrap(),
            toml::from_str(include_str!("../greenhouse/cifar_10.toml")).unwrap(),
            toml::from_str(include_str!("../greenhouse/fashion_mnist.toml")).unwrap(),
            toml::from_str(include_str!("../greenhouse/mnist.toml")).unwrap(),
            toml::from_str(include_str!("../greenhouse/trump_tweets.toml")).unwrap(),
            toml::from_str(include_str!("../greenhouse/uci_banking.toml")).unwrap(),
        ])
    }
}

impl SeedVault for GreenHouse {
    fn lookup(&self, name: &str) -> Result<Option<Pot>> {
        for pot in &self.0 {
            if pot.name == name {
                return Ok(Some(pot.clone()));
            }
        }
        Ok(None)
    }
}
