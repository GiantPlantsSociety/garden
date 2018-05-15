use std::collections::HashMap;
use std::fmt;
use serde::de;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct GardenPlan {
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
    fn parse_dependencies_from_toml() {
        let config: GardenPlan = toml::from_str(r#"
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
