use hexx::*;

#[derive(Deserialize)]
pub struct Pot {
    pub name: String,
    pub description: String,
    pub files: Vec<File>,
    pub homepage: Option<String>,
    pub reference: Option<String>,
    pub licence: Option<String>,
}

#[derive(Deserialize)]
pub struct File {
    pub url: String,
    pub sha256: Option<Hex32>,
    pub md5: Option<Hex16>,
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

            [[files]]
            url = 'http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt'
            sha256 = 'd6524d63a5cf5e5955568cc96b72b3f39258af4f0f79c61cbc01d8853e587f1b'

            [[files]]
            url = 'http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt'
            md5 = '8d4fb7e6c68d591d4c3dfef9ec88bf0a'
        "#).unwrap();

        assert_eq!(config.name, "Primes");
        assert_eq!(config.description, "These are the first 65 thousand primes. Still faster to calculate locally.");
        assert_eq!(config.files[0].url, "http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt");
        assert_eq!(config.files[0].sha256.as_ref().unwrap(), &"d6524d63a5cf5e5955568cc96b72b3f39258af4f0f79c61cbc01d8853e587f1b".parse::<Hex32>().unwrap());
        assert_eq!(config.files[1].url, "http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt");
        assert_eq!(config.files[1].md5.as_ref().unwrap(), &"8d4fb7e6c68d591d4c3dfef9ec88bf0a".parse::<Hex16>().unwrap());
    }
}
