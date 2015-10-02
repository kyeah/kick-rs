pub mod kickstarter;

pub mod schema;
pub mod table;
pub mod column;

use rustorm::table::Table;
use rustorm::table::IsTable;
use db::kickstarter::Pledge;
use db::kickstarter::Project;
use db::kickstarter::User;


pub fn get_all_tables()->Vec<Table>{
    vec![
        Pledge::table(),
        Project::table(),
        User::table(),
    ]
}