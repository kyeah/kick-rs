extern crate chrono;
extern crate codegenta;
extern crate rustc_serialize;
extern crate rustorm;
extern crate toml;

pub mod client;
pub mod pledge;
pub mod project;
pub mod gen;

pub use gen::kickstarter as models;
pub use client::Client;

use rustorm::database;
use std::{error, fmt, io, result};

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    InvalidPledge(pledge::Error),
    InvalidProject(project::Error),
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
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            Error::InvalidPledge(ref inner) => Some(inner),
            Error::InvalidProject(ref inner) => Some(inner),
            Error::Database(ref inner) => Some(inner),
            Error::IO(ref inner) => Some(inner),
        }
    }
}
