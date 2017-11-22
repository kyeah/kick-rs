use chrono::NaiveDateTime;
use models::{User, Project};
use schema::pledges;

#[derive(Debug, Clone, Identifiable, Queryable, Associations)]
#[primary_key(project_id, user_id)]
#[belongs_to(User, Project)]
pub struct Pledge {
    /// primary
    pub project_id: i32,
    /// primary
    pub user_id: i32,
    /// unique
    pub card: String,
    pub amount: f64,
    /// default: ('now'::text)::timestamp without time zone
    pub date_created: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
#[table_name="pledges"]
pub struct NewPledge {
    /// primary
    pub project_id: i32,
    /// primary
    pub user_id: i32,
    pub card: String,
    pub amount: f64,
}
