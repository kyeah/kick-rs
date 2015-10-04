//! Module for interacting with Kickstarter users.
pub use models::User;

use {Client, Result};
use db::{column, table};
use models::Pledge;

use rustorm::dao::{FromValue, Value};
use rustorm::query::{Equality, Query};
use std::cmp::Ordering;
use std::collections::BTreeMap;

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
    /// Returns a map of project names to Pledge objects.
    pub fn list_pledges(client: &Client, user: &str) -> Result<BTreeMap<String, Pledge>> {

        // Get all pledges, along with the project name.
        // Rust maps don't have ordered insertion support, so don't bother ordering by date_created.
        let dao_results = try!(Query::select()
            .column(&"pl.*")
            .column(&"pr.name")
            .from_table(&client.table_abbr(table::pledge))
            .inner_join_table(&client.table_abbr(table::user), &"pl.user_id", &"us.user_id")
            .inner_join_table(&client.table_abbr(table::project), &"pl.project_id", &"pr.project_id")
            .filter(&"us.name", Equality::EQ, &user)
            .retrieve(client.db()));

        // Map project names to the pledge data
        let mut results: BTreeMap<String, Pledge> = BTreeMap::new();
        let mut pledges: Vec<Pledge> = dao_results.cast();

        for dao in dao_results.dao.iter().rev() {
            let name = FromValue::from_type(dao.get_value(column::name));
            results.insert(name, pledges.pop().unwrap());
        }

        Ok(results)
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.user_id, &self.name).cmp(&(other.user_id, &other.name))
    }
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        (self.user_id, &self.name) == (other.user_id, &other.name)
    }
}

impl Eq for User { }
