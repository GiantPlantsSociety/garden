use std::str::FromStr;
use std::fmt;
use hex::{FromHex, FromHexError, ToHex};
use serde::de::{Deserialize, Deserializer, Error as SerdeError};

macro_rules! hex_impl {
    ( $name:ident, $size:expr ) => {
        #[derive(PartialEq, Clone)]
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

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                self.0.write_hex(f)
            }
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
                write!(f, "Hex{}(", $size)?;
                self.0.write_hex(f)?;
                write!(f, ")")?;
                Ok(())
            }
        }
    };
}

hex_impl!(Hex16, 16);
hex_impl!(Hex20, 20);
hex_impl!(Hex28, 28);
hex_impl!(Hex32, 32);
