//! Pledge-operation error module
use Result;
use std::error::Error as ErrorTrait;
use std::fmt;

/// The error type for pledge operations.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// The user name should be alphanumeric and contain only underscores or dashes.
    NameNotAlphaNumeric,
    /// The user name was not between 4 and 20 characters.
    NameLength,
    /// The credit card provided was not numeric.
    CardNotNumeric,
    /// The credit card was longer than 19 characters.
    CardLength,
    /// The credit card did not pass the Luhn-10 test.
    CardNotLuhn10,
    /// The credit card has already been used to back the desired project.
    CardExists,
    /// The pledged amount was not more than zero dollars.
    InvalidAmount,
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.description())
    }
}

impl ErrorTrait for Error {
    fn description(&self) -> &str {
        match *self {
            Error::NameNotAlphaNumeric => "User name should be alphanumeric \
                                           and contain only underscores or dashes.",
            Error::NameLength => "User name should be between 4 and 20 characters.",
            Error::CardNotNumeric => "Credit card numbers must be numeric.",
            Error::CardLength => "Credit card numbers should be between 0 and 19 characters.",
            Error::CardNotLuhn10 => "Credit card number failed the validation test.",
            Error::CardExists => "Credit card number has already been used to back this project.",
            Error::InvalidAmount => "Pledged amount must be greater than 0 dollars.",
        }
    }

    fn cause(&self) -> Option<&ErrorTrait> {
        None
    }
}
