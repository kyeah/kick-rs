//! Kickstarter data validations
pub mod error;
pub use self::error::{Error, Result};

use regex::Regex;

lazy_static! {
    static ref ALPHANUM: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    static ref NUMTEXT: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

/// Validates that the float is positive, and rounds it to two decimal places.
pub fn currency(f: f64) -> Result<f64> {
    if f <= 0.0 {
        return Err(Error::InvalidAmount);
    }

    // Convert to rounded string and reconvert back;
    // this circumvents floating-point rounding errors.
    let precise_round = format!("{:.2}", f).parse::<f64>();
    Ok(precise_round.unwrap())
}

/// Validates that the string is alphanumeric and contains only underscores and dahes as special characters.
pub fn alphanumeric(s: &str) -> Result<()> {
    regex(&ALPHANUM, s, Error::NotAlphaNumeric(s.to_owned()))
}

/// Validates that the string contains only digits.
pub fn numtext(s: &str) -> Result<()> {
    regex(&NUMTEXT, s, Error::NotNumeric(s.to_owned()))
}

/// Validates that the string matches the provided regex.
pub fn regex(reg: &Regex, s: &str, error: Error) -> Result<()> {
    if reg.is_match(s) {
        Ok(())
    } else {
        Err(error)
    }
}

/// Validates that the string length is between min and max, inclusive.
pub fn length(s: &str, min: i32, max: i32) -> Result<()> {
    let len = s.len();
    if min as usize <= len && len <= max as usize {
        Ok(())
    } else {
        Err(Error::Length(s.to_owned(), min as usize, max as usize))
    }
}

/// Validates that a numerical string passes the Luhn-10 test.
pub fn luhn10(s: &str) -> Result<()> {
    try!(numtext(s));

    // Split into reverse digit iterator
    let mut digits = s.rsplit("").filter_map(|ch| { 
        if ch.is_empty() {
            None
        } else {
            Some(ch.parse::<i8>().unwrap())
        }
    });

    let mut alt = false;
    let mut sum = 0;

    // Good ol' Luhn test
    for digit in &mut digits {
        let mut luhn = digit;
        if alt {
            luhn *= 2;
            if luhn > 9 { luhn -= 9; }
        }
        sum += luhn;
        alt = !alt;
    }

    if sum % 10 == 0 && !s.is_empty() {
        Ok(())
    } else {
        Err(Error::NotLuhn10(s.to_owned()))
    }
}
