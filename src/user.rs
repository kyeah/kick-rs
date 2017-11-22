//! Module for interacting with Kickstarter users.
pub use models::User;

use {Client, Result};
use models::{Pledge, Project};

use std::cmp::Ordering;

impl User {
    /// Upserts a user and returns the resultant ID as a Value.
    pub fn upsert(client: &Client, user: &str) -> Result<Value> {
        let u_result = try!(Query::select()
                            .column("upsert_user")
                            .from_table(&format!("upsert_user('{}')", user))
                            .retrieve_one(client.db()));
        
        let uid = u_result.values.get("upsert_user").unwrap();
        Ok(uid.clone())
    }

    /// Retrieve a map of all pledges that a user has made to Kickstarter projects.
    /// Returns a map of projects to Pledge objects.
    pub fn list_pledges(client: &Client, user: &str) -> Result<Vec<Pledge>> {

        // Get all pledges and associated projects.
        let dao_results = try!(Query::select()
            .column(&"pl.*")
            .column(&"pr.*")
            .from_table(&client.table_abbr(table::pledge))
            .inner_join_table(&client.table_abbr(table::user), &"pl.user_id", &"us.user_id")
            .inner_join_table(&client.table_abbr(table::project), &"pl.project_id", &"pr.project_id")
            .filter(&"us.name", Equality::EQ, &user)
            .retrieve(client.db()));

        // Map projects to pledges.
        let mut projects: Vec<Project> = dao_results.cast();
        let mut pledges: Vec<Pledge> = dao_results.cast();

        for i in (0..pledges.len()).rev() {
            pledges[i].project = Some(projects.pop().unwrap());
        }

        Ok(pledges)
    }
}    
