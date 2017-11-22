pub use schema::users::table;

use chrono::NaiveDateTime;
use std::cmp::Ordering;
use schema::users;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(user_id)]
pub struct User {
    /// primary
    pub user_id: i32,
    /// unique
    pub name: String,
    /// default: ('now'::text)::timestamp without time zone
    pub date_created: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name="users"]
pub struct NewUser {
    /// unique
    pub name: String,
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
