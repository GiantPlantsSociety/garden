use std::collections::HashMap;
use std::fmt;
use std::str::FromStr;
use regex::Regex;
use semver::Version;
use serde::de;
use serde::Deserialize;
use hexx::*;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct PotName(String);

impl fmt::Display for PotName {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl FromStr for PotName {
    type Err = String;

    fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
        if Regex::new(r"^[a-zA-Z0-9-_]+$").unwrap().is_match(s) {
            Ok(PotName(s.to_string()))
        } else {
            Err(format!("Bad name of a pot '{}'.", s))
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct Garden {
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Dependency {
    Version(String),
    Reference(Location),
}

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub struct Location {
    pub path: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pot {
    pub name: PotName,
    pub description: String,
    pub version: Version,
    pub files: Vec<File>,
    pub homepage: Option<String>,
    pub reference: Option<String>,
    pub licence: Option<String>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct File {
    pub url: String,
    pub md5: Option<Hex16>,
    pub sha1: Option<Hex20>,
    pub sha2_256: Option<Hex32>,
    pub sha3_224: Option<Hex28>,
    pub sha3_256: Option<Hex32>,
}

impl<'de> de::Deserialize<'de> for Dependency {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        struct DependencyVisitor;

        impl<'de> de::Visitor<'de> for DependencyVisitor {
            type Value = Dependency;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a version string like \"1.0.0\" or a \
                     reference dependency like { path = \"<local path to toml>\" }",
                )
            }

            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where
                E: de::Error,
            {
                Ok(Dependency::Version(s.to_owned()))
            }

            fn visit_map<V>(self, map: V) -> Result<Self::Value, V::Error>
            where
                V: de::MapAccess<'de>,
            {
                let mvd = de::value::MapAccessDeserializer::new(map);
                Location::deserialize(mvd).map(Dependency::Reference)
            }
        }

        deserializer.deserialize_any(DependencyVisitor)
    }
}

#[cfg(test)]
mod tests {
    use toml;
    use super::*;

    #[test]
    fn parse_primes_from_toml() {
        let config: Pot = toml::from_str(r#"
            name = 'Primes'
            description = 'These are the first 65 thousand primes. Still faster to calculate locally.'
            version = '1.2.3'

            [[files]]
            url = 'http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt'
            sha3_256 = 'd6524d63a5cf5e5955568cc96b72b3f39258af4f0f79c61cbc01d8853e587f1b'

            [[files]]
            url = 'http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt'
            md5 = '8d4fb7e6c68d591d4c3dfef9ec88bf0a'
        "#).unwrap();

        assert_eq!(config.name, "Primes".parse().unwrap());
        assert_eq!(config.description, "These are the first 65 thousand primes. Still faster to calculate locally.");
        assert_eq!(config.version, Version::new(1, 2, 3));
        assert_eq!(config.files[0].url, "http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt");
        assert_eq!(config.files[0].sha3_256.as_ref().unwrap(), &"d6524d63a5cf5e5955568cc96b72b3f39258af4f0f79c61cbc01d8853e587f1b".parse::<Hex32>().unwrap());
        assert_eq!(config.files[1].url, "http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt");
        assert_eq!(config.files[1].md5.as_ref().unwrap(), &"8d4fb7e6c68d591d4c3dfef9ec88bf0a".parse::<Hex16>().unwrap());
    }

    #[test]
    fn parse_dependencies_from_toml() {
        let config: Garden = toml::from_str(r#"
            [dependencies]
            mnist = "1.0"
            fashion_mnist = "*"
            baby_names = { path = "./baby_names_3.0.toml" }
        "#).unwrap();

        assert_eq!(config.dependencies.get("mnist").unwrap(),
            &Dependency::Version("1.0".to_string()));
        assert_eq!(config.dependencies.get("fashion_mnist").unwrap(),
            &Dependency::Version("*".to_string()));
        assert_eq!(config.dependencies.get("baby_names").unwrap(),
            &Dependency::Reference(Location { path: "./baby_names_3.0.toml".to_string() }) );
    }
}
