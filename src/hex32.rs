use std::str::FromStr;
use hex::{FromHex, FromHexError};
use serde::de::{Deserialize, Deserializer, Error as SerdeError};

#[derive(PartialEq, Debug)]
pub struct Hex32(pub [u8; 32]);

impl FromStr for Hex32 {
    type Err = FromHexError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Hex32(<[u8; 32] as FromHex>::from_hex(s)?))
    }
}

impl<'de> Deserialize<'de> for Hex32 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>
    {
        let s = String::deserialize(deserializer)?;
        let h = s.parse().map_err(SerdeError::custom)?;
        Ok(h)
    }
}
