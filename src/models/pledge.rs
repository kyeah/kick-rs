#[derive(Debug, Clone, Associations, Identifiable, Queryable)]
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
pub struct NewPledge {
    /// primary
    pub project_id: i32,
    /// primary
    pub user_id: i32,
    pub card: String,
    pub amount: f64,
}
