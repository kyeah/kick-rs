use ::{Error, Result};
use ::models::{Pledge, Project, User};

use codegenta::generator::{self, Config};
use rustorm::database::Database;
use rustorm::pool::{ManagedPool, Platform};

use std::collections::BTreeMap;
use std::fs::File;
use std::io::Read;
use toml;

const ERR_PARSING_CONFIG: &'static str = "Failed to parse configuration file";
const ERR_MISSING_URI: &'static str = "Configuration has no database connection string 'uri'";

pub struct Client {
    pub uri: String,
    pub schema: String,
    db: Platform,
}

impl Client {

    /// Creates a new Kickstarter client, connecting to the specified PostgreSQL uri and database schema.
    pub fn new(uri: &str, schema: &str) -> Result<Client> {
        let pool = try!(ManagedPool::init(uri, 1));
        let db = try!(pool.connect());
        Ok(Client {
            uri: uri.to_string(),
            schema: schema.to_string(),
            db: db,
        })
    }

    /// Creates a new Kickstarter client using the provided .toml configuration filename.
    /// If bootstrap is true, the client will wipe and recreate the database
    /// using the sql_file specified in the configuration.
    pub fn with_config(filename: &str, bootstrap: bool) -> Result<Client> {

        // Open config file
        let mut f = try!(File::open(filename));

        let mut toml = String::new();
        try!(f.read_to_string(&mut toml));

        let mut parser = toml::Parser::new(&mut toml);
        let config = match parser.parse() {
            Some(config) => config,
            None => return Err(Error::Config(format!("{}: {:?}", ERR_PARSING_CONFIG, parser.errors))),
        };

        // Read config for schema
        let schema = match config.get("schema") {
            Some(ref s) => s.as_str().unwrap(),
            None => "kickstarter",
        };

        // Retrieve and open database connection uri        
        let client = match config.get("uri") {
            Some(ref uri) => try!(Client::new(uri.as_str().unwrap(), schema)),
            None => return Err(Error::Config(ERR_MISSING_URI.to_string())),
        };

        // Bootstrap database and generated models if desired
        if bootstrap {
            let sql_file = match config.get("sql_file") {
                Some(ref s) => s.as_str().unwrap(),
                None => "data/tables.sql",
            };

            let mut sql_f = try!(File::open(sql_file));
            let mut cmds = String::new();
            try!(sql_f.read_to_string(&mut cmds));
            //client.bootstrap(&cmds);
        }

        Ok(client)
    }

    /// Returns a reference to the encapsulated database.
    pub fn db(&self) -> &Database {
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

    /// Syncs generated models in src/lib/gen with database tables.
    pub fn sync(&self) {
        let config = Config {
            base_module: Some("db".to_string()),
            include_table_references: true,
            use_condensed_name: true,
            generate_table_meta: true,
            base_dir: "./src".to_string(),
            include_views: true,
        };

        generator::generate_all(self.db.as_dev(), &config);
    }

    /// Creates a new Kickstarter project with the specified name and goal amount in US dollars.
    pub fn create_project(&self, project_name: &str, amount: f64) -> Result<()> {
        let _ = try!(Project::create(&self, project_name, amount));
        Ok(())
    }
    
    /// Backs an existing Kickstarter project with the specified user, credit card, and contribution amount.
    pub fn back_project(&self, user: &str, project_name: &str, card: &str, amount: f64) -> Result<()> {
        let _ = try!(Pledge::create(&self, user, project_name, card, amount));
        Ok(())
    }

    /// Returns a map of all backers and their contributions 
    /// towards a project, along with the project's goal amount.
    pub fn list_backers(&self, project_name: &str) -> Result<(BTreeMap<User, f64>, f64)> {
        Project::list_backers(&self, project_name)
    }

    /// Returns a list of all projects on Kickstarter.
    pub fn list_projects(&self) -> Result<Vec<Project>> {
        Project::list_all(&self)
    }

    /// Returns a map of all Kickstarter projects backed by a user, along with the pledge information.
    pub fn list_backed_projects(&self, user: &str) -> Result<BTreeMap<String, Pledge>> {
        Pledge::list_for_user(&self, user)
    }
}
