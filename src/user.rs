use ::{Client, Result};
use ::models::User;

use rustorm::dao::Value;
use rustorm::query::Query;

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
