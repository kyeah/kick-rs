use ::init_client;

use kickstarter::{project, Error};
use kickstarter::models::{Pledge, Project};
use kickstarter::db::table;

use postgres::error::SqlState;
use rustorm::dao::FromValue;
use rustorm::query::Query;

#[test]
fn create_project() {
    let client = init_client();

    let name = "GoGo_Applesauce";
    let amount = 250000f64;
    let project = Project::create(&client, name, amount).unwrap();

    assert_eq!(name, project.name);
    assert_eq!(amount, project.goal);

    let found_project: Project = Query::select_all()
        .from_table(&client.table(table::project))
        .collect_one(client.db())
        .unwrap();

    assert_eq!(name, found_project.name);
    assert_eq!(amount, found_project.goal);
}

#[test]
fn unique_name() {
    let client = init_client();

    let name = "GoGo_Applesauce";
    let amount = 250000f64;
    let _ = Project::create(&client, name, amount).unwrap();
    
    let result = Project::create(&client, name, 250f64);
    assert!(result.is_err());
    
    if let Err(Error::Database(ref err)) = result {
        assert_eq!(Some(SqlState::UniqueViolation), err.code);
    }
}

#[test]
fn get_id_by_name() {
    let client = init_client();
    
    let name = "GoGo_Applesauce";
    let amount = 250000f64;
    let project = Project::create(&client, name, amount).unwrap();

    let id = Project::get_id(&client, name).unwrap();
    assert_eq!(project.project_id, FromValue::from_type(id));
}

#[test]
fn get_id_missing() {
    let client = init_client();
    let result = Project::get_id(&client, "I_DONT_EXIST");

    match result {
        Err(Error::InvalidProject(project::Error::ProjectDoesNotExist)) => (),
        _ => panic!(result),
    }
}

#[test]
fn list_all() {
    let client = init_client();
    let names = vec!["Doritos", "Cheetos", "Exquisite_Banana"];
    let amounts = vec![1000f64, 5000.50f64, 1f64];
    
    for i in (0..3) {
        Project::create(&client, names[i], amounts[i]).unwrap();        
    }

    let projects = Project::list_all(&client).unwrap();
    assert_eq!(3, projects.len());

    for i in (0..3) {
        let index = projects.iter().position(|project| project.name == names[i]).unwrap();
        assert_eq!(amounts[i], projects[index].goal);
    }
}

#[test]
fn list_none() {
    let client = init_client();
    let projects = Project::list_all(&client).unwrap();
    assert!(projects.is_empty());
}

#[test]
fn list_backers() {
    let client = init_client();

    let name = "Seattle_Dance_Party";
    let amount = 12000f64;
    let _ = Project::create(&client, name, amount).unwrap();

    let users         = vec!["Johnnyboy", "Margie"];
    let contributions = vec![100f64, 200f64];
    let cards         = vec!["351149395124027", "6011168468345649"];
    
    for i in (0..2) {
        Pledge::create(&client, users[i], name, cards[i], contributions[i]).unwrap();
    }

    let (backers, goal) = Project::list_backers(&client, name).unwrap();
    assert_eq!(amount, goal);
    
    for (backer, contribution) in backers {
        let index = users.iter().position(|&name| name == backer.name).unwrap();
        assert_eq!(contributions[index], contribution);
    }
}

#[test]
fn list_backers_none() {
    let client = init_client();

    let name = "Alt_Dance_Owl";
    let amount = 500f64;
    let _ = Project::create(&client, name, amount).unwrap();
    let (backers, goal) = Project::list_backers(&client, name).unwrap();

    assert_eq!(amount, goal);
    assert!(backers.is_empty());
}

#[test]
fn list_backers_missing() {
    let client = init_client();
    let result = Project::list_backers(&client, "PSYCHE");
    
    match result {
        Err(Error::InvalidProject(project::Error::ProjectDoesNotExist)) => (),
        _ => panic!(result),
    }
}
