use hex32::Hex32;

#[derive(Deserialize)]
pub struct Pot {
    pub name: String,
    pub description: String,
    pub url: String,
    pub checksums: Checksums,
}

#[derive(Deserialize)]
pub struct Checksums {
    pub sha256: Hex32,
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
            url = 'http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt'

            [checksums]
            sha256 = 'd6524d63a5cf5e5955568cc96b72b3f39258af4f0f79c61cbc01d8853e587f1b'
        "#).unwrap();

        assert_eq!(config.name, "Primes");
        assert_eq!(config.description, "These are the first 65 thousand primes. Still faster to calculate locally.");
        assert_eq!(config.url, "http://staffhome.ecm.uwa.edu.au/~00061811/pub/primes.txt");
        assert_eq!(config.checksums.sha256, "d6524d63a5cf5e5955568cc96b72b3f39258af4f0f79c61cbc01d8853e587f1b".parse().unwrap());
    }
}
