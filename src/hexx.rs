use std::str::FromStr;
use hex::{FromHex, FromHexError};
use serde::de::{Deserialize, Deserializer, Error as SerdeError};

macro_rules! hex_impl {
    ( $name:ident, $size:expr ) => {
        #[derive(PartialEq, Debug, Clone)]
        pub struct $name(pub [u8; $size]);

        impl FromStr for $name {
            type Err = FromHexError;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                Ok($name(<[u8; $size] as FromHex>::from_hex(s)?))
            }
        }

        impl<'de> Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: Deserializer<'de>
            {
                let s = String::deserialize(deserializer)?;
                let h = s.parse().map_err(SerdeError::custom)?;
                Ok(h)
            }
        }
    };
}

hex_impl!(Hex16, 16);
hex_impl!(Hex32, 32);
