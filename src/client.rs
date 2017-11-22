//! The high-level client library for interacting with Kickstarter.
use {Error, Result};
use models::{Pledge, Project, User};

use std::fs::File;
use std::io::Read;
use toml;

const ERR_PARSING_CONFIG: &'static str = "Failed to parse configuration file";
const ERR_MISSING_URI:    &'static str = "Configuration has no database connection string 'uri'";
const ERR_FAILED_BUILD:   &'static str = "Failed to run one or more build commands; skipping model generation.";
const SUCCESS_BUILD:      &'static str = "Successfully built the database!";
const SUCCESS_GENERATION: &'static str = "Generated models into the db module.";

// Default configurations.
const DEFAULT_SCHEMA:     &'static str = "kickstarter";
const DEFAULT_SQL_CONFIG: &'static str = "data/tables.sql";

/// Interfaces with a Kickstarter application running on a PostgreSQL database.
pub struct Client {
    /// PostgreSQL connection URI.
    pub uri: String,
    /// Database schema name for kickstarter data.
    pub schema: String,
    /// Persistent database connection.
    db: PgConnection,
}

impl Client {

    /// Creates a new Kickstarter client, connecting to the specified PostgreSQL uri and database schema.
    pub fn new(uri: &str, schema: &str) -> Result<Client> {
        let db = try!(PgConnection::establish(&uri));
        Ok(Client {
            uri: uri.to_owned(),
            schema: schema.to_owned(),
            db: db,
        })
    }

    /// Returns a reference to the encapsulated database.
    pub fn db(&self) -> &PgConnection {
        self.db.as_ref()
    }
    
    /// Returns the full table namespace.
    pub fn table(&self, table: &str) -> String {
        format!("{}.{}", self.schema, table)
    }

    
    /// Returns the full table namespace, abbreviated with the first two letters.
    pub fn table_abbr(&self, table: &str) -> String {
        format!("{}.{} {}", self.schema, table, &table[..2])
    }

    /// Creates a new Kickstarter project with the specified name and goal amount in US dollars.
    pub fn create_project(&self, project_name: &str, amount: f64) -> Result<Project> {
        Project::create(&self, project_name, amount)
    }
    
    /// Backs an existing Kickstarter project with the specified user, credit card, and contribution amount.
    pub fn back_project(&self, user: &str, project_name: &str, card: &str, amount: f64) -> Result<Pledge> {
        Pledge::create(&self, user, project_name, card, amount)
    }

    /// Returns a a list of all pledges (and users) towards a project,
    /// along with the project's goal amount.
    pub fn list_backers(&self, project_name: &str) -> Result<(Vec<Pledge>, f64)> {
        Project::list_pledges(&self, project_name)
    }

    /// Returns a list of all projects on Kickstarter.
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        Project::list_all(&self)
    }

    /// Returns a list of all pledges (and projects) made by a user.
    pub fn list_backed_projects(&self, user: &str) -> Result<Vec<Pledge>> {
        User::list_pledges(&self, user)
    }
}
