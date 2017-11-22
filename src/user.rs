//! Module for interacting with Kickstarter users.
pub use models::User;

use {Client, Result};
use models::{Pledge, Project};

use std::cmp::Ordering;

impl User {
    /// Upserts a user and returns the resultant ID as a Value.
    // pub fn upsert(client: &Client, user: &str) -> Result<Value> {
    //     let u_result = try!(Query::select()
    //                         .column("upsert_user")
    //                         .from_table(&format!("upsert_user('{}')", user))
    //                         .retrieve_one(client.db()));
        
    //     let uid = u_result.values.get("upsert_user").unwrap();
    //     Ok(uid.clone())
    // }

    /// Retrieve a map of all pledges that a user has made to Kickstarter projects.
    /// Returns a map of projects to Pledge objects.
    pub fn list_pledges(client: &Client, user: &str) -> Result<Vec<Pledge>> {
        pledges::table.inner_join(users::table)
            .filter(users::name::eq(user))
            .load(client.db())
            .map(|&(pledges, _)| pledges)
    }
}    
