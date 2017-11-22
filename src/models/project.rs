use chrono::NaiveDateTime;
use schema::projects;

use std::cmp::Ordering;

#[derive(Debug, Clone, Identifiable, Queryable)]
#[primary_key(project_id)]
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
#[table_name="projects"]
pub struct NewProject {
    /// unique
    pub name: String,
    pub goal: f64,
}

impl Ord for Project {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.project_id, &self.name).cmp(&(other.project_id, &other.name))
    }
}

impl PartialOrd for Project {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Project {
    fn eq(&self, other: &Self) -> bool {
        (self.project_id, &self.name) == (other.project_id, &other.name)
    }
}

impl Eq for Project { }
