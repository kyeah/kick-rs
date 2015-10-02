pub mod error;

pub use self::error::Error;

use ::{project, validate, Client, Result};
use ::db::{column, table};
use ::models::{Pledge, Project, User};

use postgres::error::SqlState;

use rustorm::database::{Database, DbError};
use rustorm::query::Query;

use std::convert::From;
use std::error::Error as ErrorTrait;

impl Pledge {
    pub fn create(client: &Client, user: &str, project_name: &str, card: &str, amount: f64) -> Result<usize> {

        try!(Pledge::validate_args(user, project_name, card));

        // Validate and truncate currency amount.
        let amount = try!(validate::currency(amount, From::from(Error::InvalidAmount)));

        let uid = try!(User::upsert(client, user));
        let pid = try!(Project::get_id(client, project_name));

        // Add a new pledge.
        let mut res = Query::insert()
            .set(column::user_id, &uid)
            .set(column::project_id, &pid)
            .set(column::card, &card)
            .set(column::amount, &amount)
            .into_table(&client.table(table::pledge))
            .execute(client.db());

        Pledge::check_valid_errors(&mut res, user, project_name, card);

        let num_affected = try!(res);
        Ok(num_affected)
    }

    /// Checks pledge creation results for acceptable errors, and reformats the message.
    fn check_valid_errors(res: &mut ::std::result::Result<usize, DbError>,
                          user: &str, project_name: &str, card: &str) {

        // Check for uniqueness violations.
        let mut message = String::new();

        if let &mut Err(ref err) = res {
            if let Some(SqlState::UniqueViolation) = err.code {

                // Primary key is (user_id, project_id).
                if err.description().contains(&"pkey") {
                    message = format!("User '{}' has already backed project '{}'.", 
                                      user, project_name)

                // Only other uniqueness constraint is on the credit card,
                // but lets check the description to be sure.
                } else if err.description().contains(&"pledge_project_card") {
                    message = format!("Credit card '{}' has already been used to back project '{}'.", 
                                      card, project_name)
                }
            }
        }

        if !message.is_empty() {
            *res = Err(DbError::with_code(&message, SqlState::UniqueViolation));
        }
    }


    /// Project names must be alphanumeric and between 4 and 20 characters.
    /// User names must be alphanumeric and between 4 and 20 characters.
    /// Credit card numbers must be under 20 characters and pass the numeric & Luhn-10 tests.
    fn validate_args(user: &str, project_name: &str, card: &str) -> Result<()> {
        try!(validate::length(project_name, 4, 20, From::from(project::Error::NameLength)));
        try!(validate::length(user, 4, 20, From::from(Error::NameLength)));
        try!(validate::length(card, 1, 19, From::from(Error::CardLength)));
        try!(validate::alphanumeric(project_name, From::from(project::Error::NameNotAlphaNumeric)));
        try!(validate::alphanumeric(user, From::from(Error::NameNotAlphaNumeric)));
        try!(validate::numtext(card, From::from(Error::CardNotNumeric)));
        //try!(validate::luhn10(card));
        Ok(())
    }
}
