use std::error::Error as StdError;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::io::Error as IoError;
use std::result::Result as StdResult;

use dotenv::Error as EnvError;
use reqwest::Error as RestError;

#[derive(Debug)]
pub enum Error {
    Env(EnvError),
    Io(IoError),
    Rest(RestError),
}

pub type Result<T> = StdResult<T, Error>;

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match self {
            Error::Env(e) => write!(f, "config error: {}", e),
            Error::Io(e) => write!(f, "i/o error: {}", e),
            Error::Rest(e) => write!(f, "rest error: {}", e),
        }
    }
}

impl StdError for Error {
    fn cause(&self) -> Option<&dyn StdError> {
        match self {
            Error::Env(e) => Some(e),
            Error::Io(e) => Some(e),
            Error::Rest(e) => Some(e),
        }
    }
}

impl From<EnvError> for Error {
    fn from(err: EnvError) -> Error {
        Error::Env(err)
    }
}

impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

impl From<RestError> for Error {
    fn from(err: RestError) -> Error {
        Error::Rest(err)
    }
}
