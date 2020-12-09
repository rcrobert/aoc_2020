use std::fmt;
use std::error;
use std::result;

#[derive(Debug)]
pub struct Error {}

pub type Result<T> = result::Result<T, Error>;

impl Error {
    pub fn new() -> Self {
        Self {}
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "()")
    }
}
