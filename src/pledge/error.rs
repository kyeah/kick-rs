use ::Result;
use std::error::Error as ErrorTrait;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    NameNotAlphaNumeric,
    NameLength,
    CardNotNumeric,
    CardLength,
    CardNotLuhn10,
    CardExists,
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
            Error::NameNotAlphaNumeric => "Project name should be alphanumeric \
                                           and contain only underscores or dashes.",
            Error::NameLength => "Project name should be between 4 and 20 characters.",
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
