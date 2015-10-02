use std::fs::File;
use std::io::Read;
use toml;

use codegenta::generator::{self, Config};
use rustorm::pool::{ManagedPool, Platform};

use ::Result;
use ::models::project::{self, Project};
use ::models::user::{self, User};

pub struct Client {
    uri: String,
    db: Platform,
}

impl Client {
    pub fn new(uri: &str) -> Result<Client> {
        let pool = try!(ManagedPool::init(uri, 1));
        let db = try!(pool.connect());
        Ok(Client {
            uri: uri.to_string(),
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
            None => panic!("Failed to parse configuration: {:?}", parser.errors),
        };

        // Retrieve and open database connection uri        
        match config.get("uri") {
            Some(ref uri) => Client::new(uri.as_str().unwrap()),
            None => panic!("missing uri"),//return Error::ConfigError(ERR_MISSING_URI),
        }
    }

    /// Sync generated models in src/lib/gen with database tables.
    pub fn sync(&self) {
        let config = Config {
            base_module: Some("gen".to_string()),
            include_table_references: true,
            use_condensed_name: true,
            generate_table_meta: true,
            base_dir: "./src".to_string(),
            include_views: true,
        };

        generator::generate_all(self.db.as_dev(), &config);
    }

    pub fn create_project(&self, project_name: &str, amount: f64) -> Result<Project> {
        //let project = try!(Project::new(project_name, amount));
        // save project and return
        unimplemented!()
    }
    
    pub fn back_project(&self, user: &str, project_name: &str, card: &str, amount: f64) -> Result<()> {
        unimplemented!()
    }

    pub fn list_backers(&self, project_name: &str) -> Result<Vec<User>> {
        unimplemented!()
    }

    pub fn list_backed_projects(&self, user: &str) -> Result<Vec<Project>> {
        unimplemented!()
    }
}
