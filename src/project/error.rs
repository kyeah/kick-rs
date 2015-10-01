use ::Result;
use std::error::Error as ErrorTrait;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NameNotAlphaNumeric,
    NameLength,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NameNotAlphaNumeric => "Project name should be alphanumeric and contain only underscores or dashes.",
            Error::NameLength => "Project name should be between 4 and 20 characters.",
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        None
    }
}
