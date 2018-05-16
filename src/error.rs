use toml;
use semver;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error.")]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "Network request error.")]
    Network(#[cause] ::reqwest::Error),
    #[fail(display = "URL parse error.")]
    Parse(#[cause] ::url::ParseError),
    #[fail(display = "Directory '{}' not found.", _0)]
    DirNotFound(String),
    #[fail(display = "File '{}' not found.", _0)]
    FileNotFound(String),
    #[fail(display = "File '{}' checksum error.", _0)]
    FileChecksum(String),
    #[fail(display = "Toml config write error.")]
    TomlWriteError(toml::ser::Error),
    #[fail(display = "Toml config parse error.")]
    TomlParseError(toml::de::Error),
    #[fail(display = "Version parse error.")]
    VersionParseError(semver::ReqParseError),
    #[fail(display = "No pots named '{}' found.", _0)]
    LookupError(String),
    #[fail(display = "The latest '{}' version is v{}.", _0, _1)]
    LookupErrorWithVersionSuggestion(String, String),
}

pub type Result<T> = ::std::result::Result<T, Error>;
