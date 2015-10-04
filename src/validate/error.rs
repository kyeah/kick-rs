//! Validation error module
use Result;
use std::error::Error as ErrorTrait;
use std::fmt;

/// The error type for validation operations.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// The string should be alphanumeric and contain only underscores or dashes.
    NotAlphaNumeric(String),
    /// The string length was not within the provided bounds.
    Length(String, usize, usize),
    /// The string provided was not numeric.
    NotNumeric(String),
    /// The string did not pass the Luhn-10 test.
    NotLuhn10(String),
    /// The pledged amount was not more than zero dollars.
    InvalidAmount,
    /// The credit card has already been used to back the desired project.
    CardExists,
    /// The project being searched for does not exist.
    ProjectDoesNotExist,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::NotAlphaNumeric(ref s)  => write!(fmt, "{} should be alphanumeric \
                                                           and contain only underscores or dashes.", s),
            Error::Length(ref s, min, max) => write!(fmt, "{} must be between {} and {} characters.", s, min, max),
            Error::NotNumeric(ref s)       => write!(fmt, "{} must be numeric.", s),
            Error::NotLuhn10(ref s)        => write!(fmt, "{} failed the Luhn-10 test.", s),
            Error::InvalidAmount           => write!(fmt, "{}", self.description()),
            Error::ProjectDoesNotExist     => write!(fmt, "{}", self.description()),
            Error::CardExists              => write!(fmt, "{}", self.description()),
        }
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NotAlphaNumeric(_)  => "Argument should be alphanumeric \
                                           and contain only underscores or dashes.",
            Error::Length(..)          => "Argument length was not within the desired bounds.",
            Error::NotNumeric(_)       => "Argument must be numeric.",
            Error::NotLuhn10(_)        => "Argument failed the Luhn-10 test.",
            Error::InvalidAmount       => "Amounts must be greater than 0 dollars.",
            Error::ProjectDoesNotExist => "The project you are looking for does not exist. Go make it!",
            Error::CardExists          => "The credit card number has already been used to back this project.",
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        None
    }
}
