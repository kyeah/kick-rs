pub mod error;

pub use self::error::Error;

use ::{validate, Client, Result};
use ::db::{column, table};
use ::models::{pledge, project, Pledge, Project, User};

use postgres::error::SqlState;
use rustorm::query::{Equality, Query};
use rustorm::database::{Database, DbError};
use std::convert::From;
use std::error::Error as ErrorTrait;

impl Pledge {
    pub fn create(client: &Client, user: &str, project_name: &str, card: &str, amount: f64) -> Result<usize> {

        // Project names must be alphanumeric and between 4 and 20 characters.
        // User names must be alphanumeric and between 4 and 20 characters.
        // Credit card numbers must be under 20 characters and pass the numeric & Luhn-10 tests.
        try!(validate::length(project_name, 4, 20, From::from(::project::Error::NameLength)));
        try!(validate::length(user, 4, 20, From::from(Error::NameLength)));
        try!(validate::length(card, 1, 19, From::from(Error::CardLength)));
        try!(validate::alphanumeric(project_name, From::from(::project::Error::NameNotAlphaNumeric)));
        try!(validate::alphanumeric(user, From::from(Error::NameNotAlphaNumeric)));
        try!(validate::numtext(card, From::from(Error::CardNotNumeric)));
        //try!(validate::luhn10(card));

        // Validate and truncate currency amount.
        let amount = try!(validate::currency(amount, From::from(Error::InvalidAmount)));

        // Upsert user and retrieve ID
        let u_result = try!(Query::select()
                            .column("upsert_user")
                            .from_table(&format!("upsert_user('{}')", user))
                            .retrieve_one(client.db()));

        let uid = u_result.values.get("upsert_user").unwrap();

        // Retrieve project ID
        let p_result = try!(Query::select()
                            .column(project::project_id)
                            .from_table(&client.table(table::project))
                            .filter(project::name, Equality::EQ, &project_name)
                            .retrieve_one(client.db()));

        let pid = match p_result.values.get(column::project_id) {
            Some(ref id) => *id,
            None => return Err(From::from(Error::ProjectDoesNotExist)),
        };

        // Add a new pledge.
        let res = Query::insert()
            .set(pledge::user_id, uid)
            .set(pledge::project_id, pid)
            .set(pledge::card, &card)
            .set(pledge::amount, &amount)
            .into_table(&client.table(table::pledge))
            .execute(client.db());

        // Check for uniqueness violations.
        if let Err(ref err) = res {
            if let Some(SqlState::UniqueViolation) = err.code {

                // Primary key is (user_id, project_id).
                let message = if err.description().contains(&"pkey") {
                    format!("User '{}' has already backed project '{}'.", user, project_name)
                        
                // Only other uniqueness constraint is on the credit card,
                // but lets check the description to be sure.
                } else if err.description().contains(&"pledge_project_card") {
                    format!("Credit card '{}' has already been used to back project '{}'.", card, project_name)
                };

                return Err(From::from(DbError::with_code(
                    &message, SqlState::UniqueViolation)))
            }
        }

        let num_affected = try!(res);
        Ok(num_affected)
    }
}
