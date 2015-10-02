pub mod error;
pub use self::error::Error;

use ::Result;
use ::models::project::{self, Project};
use ::models::user::{self, User};

impl Project {
    pub fn new(project_name: String, amount: f64) -> Result<Project> {
        unimplemented!()
    }
}
