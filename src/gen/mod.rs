pub mod kickstarter;

pub mod schema;
pub mod table;
pub mod column;

use rustorm::table::Table;
use rustorm::table::IsTable;
use gen::kickstarter::Pledge;
use gen::kickstarter::Project;
use gen::kickstarter::User;


pub fn get_all_tables()->Vec<Table>{
    vec![
        Pledge::table(),
        Project::table(),
        User::table(),
    ]
}