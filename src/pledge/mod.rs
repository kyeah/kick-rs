pub mod error;

pub use self::error::Error;

use ::{validate, Client, Result};
use ::db::{column, table};
use ::models::{Pledge, Project, User};

use postgres::error::SqlState;

use rustorm::dao::FromValue;
use rustorm::database::{Database, DbError};
use rustorm::query::{Equality, Query};

use std::convert::From;
use std::collections::BTreeMap;
use std::error::Error as ErrorTrait;

impl Pledge {

    /// Creates a new pledge for an existing project.
    pub fn create(client: &Client, user: &str, project_name: &str, card: &str, amount: f64) -> Result<Pledge> {

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
        try!(validate::length(project_name, 4, 20, From::from(::project::Error::NameLength)));
        try!(validate::length(user, 4, 20, From::from(Error::NameLength)));
        try!(validate::length(card, 1, 19, From::from(Error::CardLength)));
        try!(validate::alphanumeric(project_name, From::from(::project::Error::NameNotAlphaNumeric)));
        try!(validate::alphanumeric(user, From::from(Error::NameNotAlphaNumeric)));
        try!(validate::numtext(card, From::from(Error::CardNotNumeric)));
        try!(validate::luhn10(card));
        Ok(())
    }

    /// Retrieve a map of all pledges that a user has made to Kickstarter projects.
    /// Returns a map of project names to Pledge objects.
    pub fn list_for_user(client: &Client, user: &str) -> Result<BTreeMap<String, Pledge>> {

        // Get all pledges, along with the project name.
        // Rust maps don't have ordered insertion support, so don't bother ordering by date_created.
        let dao_results = try!(Query::select()
            .column(&"pl.*")
            .column(&"pr.name")
            .from_table(&client.table_abbr(table::pledge))
            .inner_join_table(&client.table_abbr(table::user), &"pl.user_id", &"us.user_id")
            .inner_join_table(&client.table_abbr(table::project), &"pl.project_id", &"pr.project_id")
            .filter(&"us.name", Equality::EQ, &user)
            .retrieve(client.db()));

        // Map project names to the pledge data
        let mut results: BTreeMap<String, Pledge> = BTreeMap::new();
        let mut pledges: Vec<Pledge> = dao_results.cast();

        for dao in dao_results.dao.iter().rev() {
            let name = FromValue::from_type(dao.get_value(column::name));
            results.insert(name, pledges.pop().unwrap());
        }

        Ok(results)
    }
}
