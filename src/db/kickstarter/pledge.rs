//! WARNING: This file is generated, derived from table kickstarter.pledge, DO NOT EDIT

use chrono::naive::datetime::NaiveDateTime;
use db::kickstarter::Project;
use db::kickstarter::User;
use rustorm::dao::Dao;
use rustorm::dao::IsDao;
use db::schema;
use db::table;
use db::column;
use rustorm::table::IsTable;
use rustorm::table::Column;
use rustorm::table::Table;
use rustorm::table::Foreign;
use rustc_serialize::json::ToJson;
use rustc_serialize::json::Json;



#[derive(RustcDecodable, RustcEncodable)]
#[derive(Debug, Clone)]
pub struct Pledge {
    /// primary
    /// not nullable 
    /// db data type: integer
    pub project_id: i32,
    /// primary
    /// not nullable 
    /// db data type: integer
    pub user_id: i32,
    /// unique
    /// not nullable 
    /// db data type: text
    pub card: String,
    /// not nullable 
    /// db data type: double precision
    pub amount: f64,
    /// default: ('now'::text)::timestamp without time zone
    /// not nullable 
    /// db data type: timestamp without time zone
    pub date_created: NaiveDateTime,

    /// has one
    pub user: Option<User>,
    /// has one
    pub project: Option<Project>,
}



impl IsDao for Pledge {
    fn from_dao(dao: &Dao) -> Self {
        Pledge {
            user_id: dao.get(column::user_id),
            project_id: dao.get(column::project_id),
            amount: dao.get(column::amount),
            card: dao.get(column::card),
            date_created: dao.get(column::date_created),
            user: None,
            project: None,
        }
    }

    fn to_dao(&self) -> Dao {
        let mut dao = Dao::new();
        dao.set(column::user_id, &self.user_id);
        dao.set(column::project_id, &self.project_id);
        dao.set(column::amount, &self.amount);
        dao.set(column::card, &self.card);
        dao.set(column::date_created, &self.date_created);
        dao
    }
}

impl ToJson for Pledge {

    fn to_json(&self) -> Json {
        self.to_dao().to_json()
    }
}

impl IsTable for Pledge {

    fn table() -> Table {
        Table {
            schema: schema::kickstarter.to_owned(),
            name: table::pledge.to_owned(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns: vec![
                Column {
                    name: column::user_id.to_owned(),
                    data_type: "i32".to_owned(),
                    db_data_type: "integer".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: "kickstarter".to_owned(),
                            table: "user".to_owned(),
                            column: "user_id".to_owned(),
                        }),
                },
                Column {
                    name: column::project_id.to_owned(),
                    data_type: "i32".to_owned(),
                    db_data_type: "integer".to_owned(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: Some(
                        Foreign {
                            schema: "kickstarter".to_owned(),
                            table: "project".to_owned(),
                            column: "project_id".to_owned(),
                        }),
                },
                Column {
                    name: column::amount.to_owned(),
                    data_type: "f64".to_owned(),
                    db_data_type: "double precision".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::card.to_owned(),
                    data_type: "String".to_owned(),
                    db_data_type: "text".to_owned(),
                    is_primary: false, is_unique: true, not_null: true, is_inherited: false,
                    default: None,
                    comment: None,
                    foreign: None,
                },
                Column {
                    name: column::date_created.to_owned(),
                    data_type: "NaiveDateTime".to_owned(),
                    db_data_type: "timestamp without time zone".to_owned(),
                    is_primary: false, is_unique: false, not_null: true, is_inherited: false,
                    default: Some("('now'::text)::timestamp without time zone".to_owned()),
                    comment: None,
                    foreign: None,
                },
            ],
            is_view: false,
        }
    }
}
// Generated columns for easier development of dynamic queries without sacrificing wrong spelling of column names

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static user_id: &'static str = "pledge.user_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static project_id: &'static str = "pledge.project_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static amount: &'static str = "pledge.amount";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static card: &'static str = "pledge.card";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_created: &'static str = "pledge.date_created";
