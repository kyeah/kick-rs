use ::{Error, Result};
use regex::Regex;

lazy_static! {
    static ref ALPHANUM: Regex = Regex::new(r"^[a-zA-Z0-9_-]+$").unwrap();
    static ref NUMTEXT: Regex = Regex::new(r"^[0-9]+$").unwrap();
}

pub fn currency(f: f64, error: Error) -> Result<f64> {
    if f <= 0.0 {
        return Err(error)
    }

    // Convert to rounded string and reconvert back;
    // this circumvents floating-point rounding errors.
    let precise_round = format!("{:.2}", f).parse::<f64>();
    Ok(precise_round.unwrap())
}

pub fn alphanumeric(s: &str, error: Error) -> Result<()> {
    regex(&ALPHANUM, s, error)
}

pub fn numtext(s: &str, error: Error) -> Result<()> {
    regex(&NUMTEXT, s, error)
}

pub fn regex(reg: &Regex, s: &str, error: Error) -> Result<()> {
    if reg.is_match(s) {
        Ok(())
    } else {
        Err(error)
    }
}

pub fn length(s: &str, min: i32, max: i32, error: Error) -> Result<()> {
    let len = s.len();
    if min as usize <= len && len <= max as usize {
        Ok(())
    } else {
        Err(error)
    }
}
