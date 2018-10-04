use core::fmt::{self, Display};
use core::result;

use std::error;

/// An error satire.
#[derive(Debug)]
pub enum Error {
    Goblin(goblin::error::Error),
}

impl Error {
    pub fn description(&self) -> &str {
        match *self {
            Error::Goblin(ref e) => {
                e.description()
            },
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
        }
    }
}

impl Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Goblin(ref e) => write!(fmt, "Goblin error: {}", e),
        }
    }
}

pub type Result<T> = result::Result<T, Error>;
