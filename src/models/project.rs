#[derive(Debug, Clone, Identifiable, Queryable)]
pub struct Project {
    /// primary
    pub project_id: i32,
    /// unique
    pub name: String,
    pub goal: f64,
    /// default: ('now'::text)::timestamp without time zone
    pub date_created: NaiveDateTime,
}

#[derive(Debug, Clone, Insertable)]
pub struct NewProject {
    /// unique
    pub name: String,
    pub goal: f64,
}
