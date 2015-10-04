use kickstarter::{project, validate, Error};

const ERROR: Error = Error::InvalidProject(project::Error::InvalidAmount);

#[test]
fn currency_negative() {
    let result = validate::currency(-0.001f64, ERROR);
    assert!(result.is_err());
}

#[test]
fn currency_zero() {
    let result = validate::currency(0f64, ERROR);
    assert!(result.is_err());
}

#[test]
fn currency_valid_rounded() {
    let result = validate::currency(0.114f64, ERROR).unwrap();
    assert_eq!(result, 0.11f64);
}

#[test]
fn currency_valid() {
    let result = validate::currency(1.99f64, ERROR).unwrap();
    assert_eq!(result, 1.99f64);
}

#[test]
fn alphanumeric_non_alphanum() {
    let invalid = vec![
        "Shakey Graves",
        "abcde*",
        "_!__",
        "$",
    ];

    for s in &invalid {
        let result = validate::alphanumeric(s, ERROR);
        assert!(result.is_err());
    }
}

#[test]
fn alphanumeric_valid_underscores_dashes() {
    let valid = vec![
        "abcde_-",
        "-edcba",
        "ab_cd-_e",
        "_saved-by-the-bell_",
    ];
    
    for s in &valid {
        let result = validate::alphanumeric(s, ERROR);
        assert!(result.is_ok());
    }
}

#[test]
fn alphanumeric_valid() {
    let result = validate::alphanumeric("helloKickstarter", ERROR);
    assert!(result.is_ok());
}

#[test]
fn numtext_non_numtext() {
    let invalid = vec![
        "b",
        "1235125a",
        "d2412429301",
        "2342b3351",
        "434151_",
        "2412-21512"
    ];

    for s in &invalid {
        let result = validate::numtext(s, ERROR);
        assert!(result.is_err());
    }
}

#[test]
fn numtext_valid() {
    let valid = vec![
        "12312",
        "1241251253",
        "129571257124",
        "9996473459912905712075019750174012573"
    ];

    for s in &valid {
        let result = validate::numtext(s, ERROR);
        assert!(result.is_ok());
    }
}

#[test]
fn length_low() {
    let invalid = vec![
        "",
        "h",
        "he",
        "hello"
    ];

    for s in &invalid {
        let result = validate::length(s, 6, 10, ERROR);
        assert!(result.is_err());
    }
}

#[test]
fn length_high() {
    let invalid = vec![
        "supercalifr",
        "supercalifragilistic",
        "supercalifragilisticexpialidocious"
    ];

    for s in &invalid {
        let result = validate::length(s, 6, 10, ERROR);
        assert!(result.is_err());
    }
}

#[test]
fn length_valid() {
    let valid = vec![
        "helloo",
        "helloooo",
        "helloooooo"
    ];

    for s in &valid {
        let result = validate::length(s, 6, 10, ERROR);
        assert!(result.is_ok());
    }
}

#[test]
fn luhn10_non() {
    let invalid = vec![
        "",
        "4298758533045499",
        "4578423560846672",
        "3414432760899",
        "3395124027",
        "60111568345649",
        "381390580",
    ];

    for card in &invalid {
        let result = validate::luhn10(card);
        assert!(result.is_err());
    }
}

#[test]
fn luhn10_valid() {
    let valid = vec![
        "4298708533045499",
        "4248146957095198",
        "4578423530846672",
        "341468752760899",
        "351149395124027",
        "6011168468345649"
    ];

    for card in &valid {
        let result = validate::luhn10(card);
        assert!(result.is_ok());
    }
}
