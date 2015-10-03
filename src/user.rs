use ::{Client, Result};
use ::models::User;

use rustorm::dao::Value;
use rustorm::query::Query;
use std::cmp::Ordering;

impl User {
    /// Upsert user and retrieve the resultant ID.
    pub fn upsert(client: &Client, user: &str) -> Result<Value> {
        // Upsert user and retrieve ID
        let u_result = try!(Query::select()
                            .column("upsert_user")
                            .from_table(&format!("upsert_user('{}')", user))
                            .retrieve_one(client.db()));
        
        let uid = u_result.values.get("upsert_user").unwrap();
        Ok(uid.clone())
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
