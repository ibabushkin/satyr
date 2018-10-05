use core::fmt::{self, Display};
use core::result;

use std::{error, io};

/// An error satire.
#[derive(Debug)]
pub enum Error {
    Goblin(goblin::error::Error),
    IO(io::Error),
    UnsupportedPlatform,
}

impl Error {
    pub fn description(&self) -> &str {
        match *self {
            Error::Goblin(ref e) => e.description(),
            Error::IO(ref e) => std::error::Error::description(e),
            Error::UnsupportedPlatform => "An ELF file's platform is not supported",
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.description()
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::Goblin(ref e) => e.cause(),
            Error::IO(ref e) => e.cause(),
            Error::UnsupportedPlatform => None,
        }
    }
}

impl From<goblin::error::Error> for Error {
    fn from(e: goblin::error::Error) -> Error {
        Error::Goblin(e)
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::IO(e)
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Goblin(ref e) => write!(fmt, "Goblin error: {}", e),
            Error::IO(ref e) => write!(fmt, "IO error: {}", e),
            Error::UnsupportedPlatform => write!(fmt, "Unsupported platform of object file"),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
