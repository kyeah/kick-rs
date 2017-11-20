#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct User {
    /// primary
    pub user_id: i32,
    /// unique
    pub name: String,
    /// default: ('now'::text)::timestamp without time zone
    pub date_created: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
pub struct NewUser {
    /// unique
    pub name: String,
}
