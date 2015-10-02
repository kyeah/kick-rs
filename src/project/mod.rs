pub mod error;

pub use self::error::Error;

use ::{validate, Client, Result};
use ::db::{column, table};
use ::models::Project;

use postgres::error::SqlState;

use rustorm::dao::Value;
use rustorm::database::{Database, DbError};
use rustorm::query::{Equality, Query};

use std::convert::From;

impl Project {
    pub fn create(client: &Client, project_name: &str, amount: f64) -> Result<usize> {

        // Names must be alphanumeric and between 4 & 20 characters.
        try!(validate::length(project_name, 4, 20, From::from(Error::NameLength)));
        try!(validate::alphanumeric(project_name, From::from(Error::NameNotAlphaNumeric)));

        // Validate and truncate the currency amount.
        let amount = try!(validate::currency(amount, From::from(Error::InvalidAmount)));

        // Attempt to insert project into the table...
        let mut result = Query::insert()
            .set(column::name, &project_name)
            .set(column::goal, &amount)
            .into_table(&client.table(table::project))
            .execute(client.db());

        // and catch uniqueness violations to return a custom error.
        Project::check_valid_errors(&mut result, project_name);

        let num_affected = try!(result);
        Ok(num_affected)
    }

    /// Checks project creation results for acceptable errors, and reformats the message.
    fn check_valid_errors(res: &mut ::std::result::Result<usize, DbError>, project_name: &str) {

        let mut message = String::new();        

        if let &mut Err(ref err) = res {
            if let Some(SqlState::UniqueViolation) = err.code {
                message = format!("Project '{}' already exists!", project_name);
            }
        }

        if !message.is_empty() {
            *res = Err(DbError::with_code(&message, SqlState::UniqueViolation));
        }
    }

    /// Retrieve a project ID by name.
    pub fn get_id(client: &Client, project_name: &str) -> Result<Value> {
        let p_result = try!(Query::select()
                            .column(column::project_id)
                            .from_table(&client.table(table::project))
                            .filter(column::name, Equality::EQ, &project_name)
                            .retrieve_one(client.db()));

        match p_result.values.get(column::project_id) {
            Some(ref id) => Ok((*id).clone()),
            None => Err(From::from(Error::ProjectDoesNotExist)),
        }
    }
}
