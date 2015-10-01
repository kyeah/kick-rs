pub mod error;
pub use self::error::Error;

use ::Result;
use ::models::project::{self, Project};
use ::models::user::{self, User};

pub fn create(project_name: String, amount: f64) -> Result<Project> {
    unimplemented!()
}

pub fn list_backers(project_name: String) -> Result<Vec<User>> {
    unimplemented!()
}
