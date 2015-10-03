//! WARNING: This file is generated, derived from table kickstarter.user, DO NOT EDIT

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
pub struct User {
    /// primary
    /// default: nextval('kickstarter.user_user_id_seq'::regclass)
    /// not nullable 
    /// db data type: integer
    pub user_id: i32,
    /// unique
    /// not nullable 
    /// db data type: text
    pub name: String,
    /// default: ('now'::text)::timestamp without time zone
    /// not nullable 
    /// db data type: timestamp without time zone
    pub date_created: NaiveDateTime,

    /// has many
    pub pledge: Vec<Pledge>,
}



impl IsDao for User{
    fn from_dao(dao:&Dao)->Self{
        User{
            user_id: dao.get(column::user_id),
            name: dao.get(column::name),
            date_created: dao.get(column::date_created),
            pledge: vec![],
        }
    }

    fn to_dao(&self)->Dao{
        let mut dao = Dao::new();
        dao.set(column::user_id, &self.user_id);
        dao.set(column::name, &self.name);
        dao.set(column::date_created, &self.date_created);
        dao
    }
}

impl ToJson for User{

    fn to_json(&self)->Json{
        self.to_dao().to_json()
    }
}

impl IsTable for User{

    fn table()->Table{
    
        Table{
            schema: schema::kickstarter.to_string(),
            name: table::user.to_string(),
            parent_table: None,
            sub_table: vec![],
            comment: None,
            columns:
            vec![
                Column{
                    name: column::user_id.to_string(),
                    data_type: "i32".to_string(),
                    db_data_type: "integer".to_string(),
                    is_primary: true, is_unique: false, not_null: true, is_inherited: false, 
                    default:Some("nextval('kickstarter.user_user_id_seq'::regclass)".to_string()),
                    comment:None,
                    foreign: None,
                },
                Column{
                    name: column::name.to_string(),
                    data_type: "String".to_string(),
                    db_data_type: "text".to_string(),
                    is_primary: false, is_unique: true, not_null: true, is_inherited: false, 
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
pub static user_id: &'static str = "user.user_id";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static name: &'static str = "user.name";

#[allow(non_upper_case_globals)]
#[allow(dead_code)]
pub static date_created: &'static str = "user.date_created";
