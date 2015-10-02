use ::Result;
use std::error::Error as ErrorTrait;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NameNotAlphaNumeric,
    NameLength,
    InvalidAmount,
    ProjectDoesNotExist,
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
            Error::InvalidAmount => "Goal amount must be greater than 0 dollars.",
            Error::ProjectDoesNotExist => "The project you are trying to back does not exist :(",
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        None
    }
}
