pub mod error;
pub use self::error::Error;

use ::Result;
use ::models::project::{self, Project};

pub fn create(user: String, project_name: String, card: String, amount: f64) -> Result<()> {
    unimplemented!()
}

pub fn list_backed(user: String) -> Result<Vec<Project>> {
    unimplemented!()
}
