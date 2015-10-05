//! Module for interacting with Kickstarter projects.
pub use models::Project;

use {validate, Client, Result};
use db::{column, table};
use models::{Pledge, User};

use postgres::error::SqlState;
use rustorm::dao::{FromValue, Value};
use rustorm::database::DbError;
use rustorm::query::{Equality, Query};

use std::cmp::Ordering;
use std::convert::From;

impl Project {

    /// Creates a new Kickstarter project with the provided goal amount in dollars.
    /// Returns the created project on success.
    pub fn create(client: &Client, project_name: &str, amount: f64) -> Result<Project> {

        // Names must be alphanumeric and between 4 & 20 characters.
        try!(validate::length(project_name, 4, 20));
        try!(validate::alphanumeric(project_name));

        // Validate and truncate the currency amount.
        let amount = try!(validate::currency(amount));

        // Attempt to insert project into the table...
        let mut result = Query::insert()
            .set(column::name, &project_name)
            .set(column::goal, &amount)
            .into_table(&client.table(table::project))
            .return_all()
            .collect_one(client.db());

        // and catch uniqueness violations to return a custom error.
        Project::check_valid_errors(&mut result, project_name);

        let project = try!(result);
        Ok(project)
    }

    /// Checks project creation results for acceptable errors, and reformats the message.
    fn check_valid_errors(res: &mut ::std::result::Result<Project, DbError>, project_name: &str) {

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
        let result = try!(Query::select()
                          .column(column::project_id)
                          .from_table(&client.table(table::project))
                          .filter(column::name, Equality::EQ, &project_name)
                          .retrieve(client.db()));

        if result.dao.is_empty() {
            Err(From::from(validate::Error::ProjectDoesNotExist))
        } else {
            let id = result.dao[0].values.get(column::project_id).unwrap();
            Ok(id.clone())
        }
    }

    /// Returns a list of all projects on Kickstarter.
    pub fn list_all(client: &Client) -> Result<Vec<Project>> {
        let results: Vec<Project> = try!(Query::select_all()
            .from_table(&client.table(table::project))
            .collect(client.db()));

        Ok(results)
    }

    /// Retrieves a list of all pledges for a given project. Returns a list of 
    /// all pledges with user information, as well as the overall project goal amount.
    pub fn list_pledges(client: &Client, project_name: &str) -> Result<(Vec<Pledge>, f64)> {
        let mut dao_results = try!(Query::select()
            .column(&"us.*")
            .column(&"pl.*")
            .column(&"pr.goal")
            .from_table(&client.table_abbr(table::project))
            .left_join_table(&client.table_abbr(table::pledge), &"pl.project_id", &"pr.project_id")
            .left_join_table(&client.table_abbr(table::user), &"pl.user_id", &"us.user_id")
            .filter(&"pr.name", Equality::EQ, &project_name)
            .retrieve(client.db()));

        if dao_results.dao.is_empty() {
            return Err(From::from(validate::Error::ProjectDoesNotExist));
        }

        let goal = dao_results.dao[0].get_value(column::goal);        

        dao_results.dao.retain(|dao| {
            dao.get_value(column::amount) != Value::Null
        });

        // Map users to pledges
        let mut users: Vec<User> = dao_results.cast();
        let mut pledges: Vec<Pledge> = dao_results.cast();

        for i in (0..pledges.len()).rev() {
            pledges[i].user = Some(users.pop().unwrap());
        }

        Ok((pledges, FromValue::from_type(goal)))
    }
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.project_id, &self.name).cmp(&(other.project_id, &other.name))
    }
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        (self.project_id, &self.name) == (other.project_id, &other.name)
    }
}

impl Eq for Project { }
