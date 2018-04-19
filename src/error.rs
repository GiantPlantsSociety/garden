#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Epic fail.")]
    EpicFail,
}

pub type Result<T> = ::std::result::Result<T, Error>;
