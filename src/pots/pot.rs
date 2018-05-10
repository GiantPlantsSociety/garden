use std::collections::HashMap;
use semver::{Version, VersionReq};
use hexx::*;

#[derive(Deserialize, Debug, Clone)]
pub struct Garden {
    pub dependencies: HashMap<String, VersionReq>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Pot {
    pub name: String,
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

        assert_eq!(config.name, "Primes");
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
        "#).unwrap();

        assert!(config.dependencies.get("mnist").unwrap().matches(&Version::new(1, 0, 0)));
        assert!(config.dependencies.get("fashion_mnist").unwrap().matches(&Version::new(1, 2, 3)));
    }
}
