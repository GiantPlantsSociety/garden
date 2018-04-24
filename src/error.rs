#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "I/O error.")]
    Io(#[cause] ::std::io::Error),
    #[fail(display = "Network request error.")]
    Network(#[cause] ::reqwest::Error),
    #[fail(display = "URL parse error.")]
    Parse(#[cause] ::url::ParseError),
    #[fail(display = "File '{}' not found.", _0)]
    FileNotFound(String)
}

pub type Result<T> = ::std::result::Result<T, Error>;
