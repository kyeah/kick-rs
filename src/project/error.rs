use ::Result;
use std::error::Error as ErrorTrait;
use std::fmt;

/// The error type for project operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The project name must be alphanumeric and contain only underscores or dashes.
    NameNotAlphaNumeric,
    /// The project name was not between 4 and 20 characters.
    NameLength,
    /// The goal amount was not more than zero dollars.
    InvalidAmount,
    /// The project being searched for does not exist.
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
            Error::NameNotAlphaNumeric => "Project name should be alphanumeric \
                                           and contain only underscores or dashes.",
            Error::NameLength => "Project name should be between 4 and 20 characters.",
            Error::InvalidAmount => "Goal amount must be greater than 0 dollars.",
            Error::ProjectDoesNotExist => "The project you are looking for does not exist. Go make it!",
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        None
    }
}
