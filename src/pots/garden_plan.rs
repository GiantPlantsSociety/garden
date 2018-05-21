use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use url::Url;
use url_serde;
use serde::de::{self, Error};
use serde::Deserialize;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct GardenPlan {
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum Dependency {
    Version(String),
    Local { path: PathBuf },
    Remote { #[serde(with = "url_serde")] url: Url },
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
                #[derive(Deserialize)]
                struct DependencyInternal {
                    path: Option<PathBuf>,
                    url: Option<url_serde::Serde<Url>>,
                }

                let mvd = de::value::MapAccessDeserializer::new(map);
                let internal = DependencyInternal::deserialize(mvd)?;
                match internal {
                    DependencyInternal { path: Some(path), url: None } => Ok(Dependency::Local { path }),
                    DependencyInternal { path: None, url: Some(url) } => Ok(Dependency::Remote { url: url.into_inner() }),
                    DependencyInternal { path: Some(..), url: Some(..) } => Err(V::Error::custom("`path` and `url` are mutually exclusive")),
                    DependencyInternal { path: None, url: None } => Err(V::Error::custom("`path` or `url` should be specified")),
                }
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
    fn parse_dependencies_from_toml() {
        let config: GardenPlan = toml::from_str(r#"
            [dependencies]
            mnist = "1.0"
            fashion_mnist = "*"
            baby_names = { path = "./baby_names_3.0.toml" }
            example = { url = "https://example.com/data/example_1.2.toml" }
        "#).unwrap();

        assert_eq!(config.dependencies.get("mnist").unwrap(),
            &Dependency::Version("1.0".to_string()));
        assert_eq!(config.dependencies.get("fashion_mnist").unwrap(),
            &Dependency::Version("*".to_string()));
        assert_eq!(config.dependencies.get("baby_names").unwrap(),
            &Dependency::Local { path: PathBuf::from("./baby_names_3.0.toml") } );
        assert_eq!(config.dependencies.get("example").unwrap(),
            &Dependency::Remote { url: Url::parse("https://example.com/data/example_1.2.toml").unwrap() } );
    }

    #[test]
    fn parse_dependencies_from_toml_with_empty_dependency() {
        let maybe_config: Result<GardenPlan, _> = toml::from_str(r#"
            [dependencies]
            baby_names = { checksum = "3" }
        "#);
        assert!(maybe_config.is_err());
        assert_eq!(format!("{}", maybe_config.unwrap_err()), "`path` or `url` should be specified for key `dependencies.baby_names`");
    }

    #[test]
    fn parse_dependencies_from_toml_with_bad_dependency() {
        let maybe_config: Result<GardenPlan, _> = toml::from_str(r#"
            [dependencies]
            baby_names = { path = "./baby_names_3.0.toml", url = "https://example.com/data/example_1.2.toml" }
        "#);
        assert!(maybe_config.is_err());
        assert_eq!(format!("{}", maybe_config.unwrap_err()), "`path` and `url` are mutually exclusive for key `dependencies.baby_names`");
    }
}
