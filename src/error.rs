use ::{pledge, project};
use rustorm::database;
use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidPledge(pledge::Error),
    InvalidProject(project::Error),
    Config(String),
    Database(database::DbError),
    IO(io::Error),
}

impl<'a> From<pledge::Error> for Error {
    fn from(err: pledge::Error) -> Error {
        Error::InvalidPledge(err)
    }
}

impl<'a> From<project::Error> for Error {
    fn from(err: project::Error) -> Error {
        Error::InvalidProject(err)
    }
}

impl<'a> From<database::DbError> for Error {
    fn from(err: database::DbError) -> Error {
        Error::Database(err)
    }
}

impl<'a> From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IO(err)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::InvalidPledge(ref inner) => inner.fmt(fmt),            
            Error::InvalidProject(ref inner) => inner.fmt(fmt),
            Error::Database(ref inner) => inner.fmt(fmt),
            Error::IO(ref inner) => inner.fmt(fmt),
            Error::Config(ref inner) => inner.fmt(fmt),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::InvalidPledge(ref inner) => inner.description(),
            Error::InvalidProject(ref inner) => inner.description(),
            Error::Database(ref inner) => inner.description(),
            Error::IO(ref inner) => inner.description(),
            Error::Config(ref inner) => inner,
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidPledge(ref inner) => Some(inner),
            Error::InvalidProject(ref inner) => Some(inner),
            Error::Database(ref inner) => Some(inner),
            Error::IO(ref inner) => Some(inner),
            Error::Config(_) => None,
        }
    }
}
