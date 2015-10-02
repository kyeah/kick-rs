//! WARNING: This file is generated, derived from table kickstarter.project, DO NOT EDIT

use chrono::naive::datetime::NaiveDateTime;
use db::kickstarter::Pledge;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use db::schema;
use db::table;
use db::column;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Project {
    /// primary
    /// default: nextval('kickstarter.project_project_id_seq'::regclass)
    /// not nullable 
    /// db data type: integer
    pub project_id: i32,
    /// unique
    /// not nullable 
    /// db data type: text
    pub name: String,
    /// default: ('now'::text)::timestamp without time zone
    /// not nullable 
    /// db data type: timestamp without time zone
    pub date_created: NaiveDateTime,
    /// not nullable 
    /// db data type: double precision
    pub goal: f64,

    /// has many
    pub pledge: Vec<Pledge>,
}



impl IsDao for Project{
    fn from_dao(dao:&Dao)->Self{
        Project{
            project_id: dao.get(column::project_id),
            name: dao.get(column::name),
            goal: dao.get(column::goal),
            date_created: dao.get(column::date_created),
            pledge: vec![],
        }
    }

    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        dao.set(column::project_id, &self.project_id);
        dao.set(column::name, &self.name);
        dao.set(column::goal, &self.goal);
        dao.set(column::date_created, &self.date_created);
        dao
    }
}

impl ToJson for Project{

    fn to_json(&self)->Json{
        self.to_dao().to_json()
    }
}

impl IsTable for Project{

    fn table()->Table{
    
        Table{
            schema: schema::kickstarter.to_string(),
            name: table::project.to_string(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns:
            vec![
                Column{
                    name: column::project_id.to_string(),
                    data_type: "i32".to_string(),
                    db_data_type: "integer".to_string(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false, 
                    default:Some("nextval('kickstarter.project_project_id_seq'::regclass)".to_string()),
                    comment:None,
                    foreign: None,
                },
                Column{
                    name: column::name.to_string(),
                    data_type: "String".to_string(),
                    db_data_type: "text".to_string(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false, 
                    default:None,
                    comment:None,
                    foreign: None,
                },
                Column{
                    name: column::goal.to_string(),
                    data_type: "f64".to_string(),
                    db_data_type: "double precision".to_string(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false, 
                    default:None,
                    comment:None,
                    foreign: None,
                },
                Column{
                    name: column::date_created.to_string(),
                    data_type: "NaiveDateTime".to_string(),
                    db_data_type: "timestamp without time zone".to_string(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false, 
                    default:Some("('now'::text)::timestamp without time zone".to_string()),
                    comment:None,
                    foreign: None,
                },
            ],
            is_view: false
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static project_id: &'static str = "project.project_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "project.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static goal: &'static str = "project.goal";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_created: &'static str = "project.date_created";
