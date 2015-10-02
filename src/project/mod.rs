pub mod error;

pub use self::error::Error;

use ::{validate, Client, Result};
use ::models::project::{self, Project};
use ::db::table;

use postgres::error::SqlState;
use rustorm::database::{Database, DbError};
use rustorm::query::Query;
use std::convert::From;
use std::error::Error as ErrorTrait;

impl Project {
    pub fn create(client: &Client, project_name: &str, amount: f64) -> Result<usize> {

        // Names must be alphanumeric and between 4 & 20 characters.
        try!(validate::length(project_name, 4, 20, From::from(Error::NameLength)));
        try!(validate::alphanumeric(project_name, From::from(Error::NameNotAlphaNumeric)));

        // Validate and truncate the currency amount.
        let amount = try!(validate::currency(amount, From::from(Error::InvalidAmount)));

        // Attempt to insert project into the table...
        let result = Query::insert()
                      .set(project::name, &project_name)
                      .set(project::goal, &amount)
                      .into_table(&client.table(table::project))
                      .execute(client.db());

        // and catch uniqueness violations to return a custom error.
        if let Err(ref err) = result {
            if let Some(SqlState::UniqueViolation) = err.code {
                return Err(From::from(DbError::with_code(
                    &format!("Project '{}' already exists!", project_name), SqlState::UniqueViolation)))
            }
        }

        let num_affected = try!(result);
        Ok(num_affected)
    }
}
