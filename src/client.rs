use std::fs::File;
use std::io::Read;
use toml;

use codegenta::generator::{self, Config};
use rustorm::database::Database;
use rustorm::pool::{ManagedPool, Platform};

use ::{Error, Result};
use ::models::{Pledge, Project, User};

const ERR_PARSING_CONFIG: &'static str = "Failed to parse configuration file";
const ERR_MISSING_URI: &'static str = "Configuration has no database connection string 'uri'";

pub struct Client {
    pub uri: String,
    pub schema: String,
    db: Platform,
}

impl Client {
    pub fn new(uri: &str, schema: &str) -> Result<Client> {
        let pool = try!(ManagedPool::init(uri, 1));
        let db = try!(pool.connect());
        Ok(Client {
            uri: uri.to_string(),
            schema: schema.to_string(),
            db: db,
        })
    }

    pub fn with_config(filename: &str) -> Result<Client> {
        // Open config file
        let mut f = try!(File::open(filename));

        let mut toml = String::new();
        try!(f.read_to_string(&mut toml));

        let mut parser = toml::Parser::new(&mut toml);
        let config = match parser.parse() {
            Some(config) => config,
            None => return Err(Error::Config(format!("{}: {:?}", ERR_PARSING_CONFIG, parser.errors))),
        };

        let schema = match config.get("schema") {
            Some(ref s) => s.as_str().unwrap(),
            None => "kickstarter",
        };

        // Retrieve and open database connection uri        
        match config.get("uri") {
            Some(ref uri) => Client::new(uri.as_str().unwrap(), schema),
            None => return Err(Error::Config(ERR_MISSING_URI.to_string())),
        }
    }

    pub fn db(&self) -> &Database {
        self.db.as_ref()
    }
    
    // Return the full table namespace
    pub fn table(&self, table: &str) -> String {
        format!("{}.{}", self.schema, table)
    }

    /// Sync generated models in src/lib/gen with database tables.
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

    pub fn create_project(&self, project_name: &str, amount: f64) -> Result<usize> {
        let num_affected = try!(Project::create(&self, project_name, amount));
        println!("Added project '{}' with a target goal of ${:.2}", project_name, amount);
        Ok(num_affected)
    }
    
    pub fn back_project(&self, user: &str, project_name: &str, card: &str, amount: f64) -> Result<()> {
        let _ = try!(Pledge::create(&self, user, project_name, card, amount));
        println!("Backed project '{}' with a target goal of ${:.2}", project_name, amount);
        Ok(())
    }

    pub fn list_backers(&self, project_name: &str) -> Result<Vec<User>> {
        unimplemented!()
    }

    pub fn list_backed_projects(&self, user: &str) -> Result<Vec<Project>> {
        unimplemented!()
    }
}
