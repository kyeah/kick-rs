//! Module for interacting with Kickstarter projects.
use {validatPledgee, Client, Result};
use models::{NewProject, Project, Pledge, User};
use schema::projects;

use diesel::{self, BelongingToDsl, ExpressionMethods, FilterDsl, FindDsl, FirstDsl};
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
        let new_project = NewProject {
            name: project_name,
            goal: amount,
        };

        let mut result = diesel::insert(&new_project)
            .into(projects::table)
            .get_result(client.db());

        // and catch uniqueness violations to return a custom error.
        Project::check_valid_errors(&mut result, project_name);

        let project = try!(result);
        Ok(project)
    }

    /// Checks project creation results for acceptable errors, and reformats the message.
    fn check_valid_errors(res: &mut ::std::result::Result<Project, diesel::result::Error>, project_name: &str) {
        let mut message = String::new();        

        if let &mut Err(DatabaseError(UniqueViolation, _)) = res {
            message = format!("Project '{}' already exists!", project_name);
        }

        if !message.is_empty() {
            *res = Err(DbError::with_code(&message, SqlState::UniqueViolation));
        }
    }

    /// Retrieve a project ID by name.
    pub fn get_id(client: &Client, project_name: &str) -> Result<i32> {
        projects::select(projects::project_id)
            .filter(projects::name.eq(project_name))
            .first(client.db())
            .map_err(|_| Err(From::from(validate::Error::ProjectDoesNotExist)))
    }

    /// Returns a list of all projects on Kickstarter.
    pub fn list_all(client: &Client) -> Result<Vec<Project>> {
        projects::load(client.db())
    }

    /// Retrieves a list of all pledges for a given project. Returns a list of 
    /// all pledges with user information, as well as the overall project goal amount.
    pub fn list_pledges(client: &Client, project_name: &str) -> Result<(Vec<Pledge>, f64)> {
        let project = try!(projects::table.find(1).filter(projects::name.eq(project_name)).first(client.db()));
        let pledges = try!(Pledge::belonging_to(&project).get_results(client.db()));
        Ok((pledges, project.goal))
    }
}
