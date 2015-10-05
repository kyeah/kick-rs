//! Module for interacting with Kickstarter pledges.
pub use models::Pledge;

use {validate, Client, Result};
use db::{column, table};
use models::{Project, User};

use postgres::error::SqlState;
use rustorm::database::DbError;
use rustorm::query::Query;

use std::error::Error as ErrorTrait;

impl Pledge {

    /// Returns a reference to the user that made the pledge.
    pub fn get_user(&self) -> &User {
        self.user.as_ref().unwrap()
    }
    
    /// Returns a reference to the project that the pledge is for.
    pub fn get_project(&self) -> &Project {
        self.project.as_ref().unwrap()
    }

    /// Creates a new pledge for an existing project.
    pub fn create(client: &Client, user: &str, project_name: &str, card: &str, amount: f64) -> Result<Pledge> {

        try!(Pledge::validate_args(user, project_name, card));

        // Validate and truncate currency amount.
        let amount = try!(validate::currency(amount));

        let uid = try!(User::upsert(client, user));
        let pid = try!(Project::get_id(client, project_name));

        // Add a new pledge.
        let mut res = Query::insert()
            .set(column::user_id, &uid)
            .set(column::project_id, &pid)
            .set(column::card, &card)
            .set(column::amount, &amount)
            .into_table(&client.table(table::pledge))
            .return_all()
            .collect_one(client.db());

        Pledge::check_valid_errors(&mut res, user, project_name, card);

        let pledge = try!(res);
        Ok(pledge)
    }

    /// Checks pledge creation results for acceptable errors, and reformats the message.
    fn check_valid_errors(res: &mut ::std::result::Result<Pledge, DbError>,
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
        try!(validate::length(project_name, 4, 20));
        try!(validate::length(user, 4, 20));
        try!(validate::length(card, 1, 19));
        try!(validate::alphanumeric(project_name));
        try!(validate::alphanumeric(user));
        try!(validate::luhn10(card));
        Ok(())
    }
}
