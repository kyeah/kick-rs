use ::{init_client, init_test_projects, init_test_pledges, 
       NAMES, GOALS, USERS, CONTRIBUTIONS, NUM_PROJECTS};

use kickstarter::{validate, Error};
use kickstarter::models::Project;
use kickstarter::db::table;

use postgres::error::SqlState;
use rustorm::dao::FromValue;
use rustorm::query::Query;

#[test]
fn create_project() {
    let (client, projects) = init_test_projects();
    
    // Check that all Project::create calls returned the right information.
    for i in (0..NUM_PROJECTS) {
        let ref project = projects[i];
        assert_eq!(NAMES[i], project.name);
        assert_eq!(GOALS[i], project.goal);
    }

    // Query and cross-check with the expected information.
    let found_projects: Vec<Project> = Query::select_all()
        .from_table(&client.table(table::project))
        .collect(client.db())
        .unwrap();

    assert_eq!(NUM_PROJECTS, found_projects.len());

    for i in (0..NUM_PROJECTS) {
        // Check that each name is found within the queried projects vector.
        let index = found_projects.iter().position(|project| project.name == NAMES[i]).unwrap();

        // Check that the associated goal is correct.
        assert_eq!(GOALS[i], found_projects[index].goal);
    }
}

#[test]
fn unique_name() {
    let (client, _) = init_test_projects();
    let result = Project::create(&client, NAMES[0], 250f64);
    assert!(result.is_err());

    if let Err(Error::Database(ref err)) = result {
        assert_eq!(Some(SqlState::UniqueViolation), err.code);
    }
}

#[test]
fn get_id_by_name() {
    let (client, projects) = init_test_projects();
    let id = Project::get_id(&client, NAMES[0]).unwrap();
    assert_eq!(projects[0].project_id, FromValue::from_type(id));
}

#[test]
fn get_id_missing() {
    let client = init_client();
    let result = Project::get_id(&client, "I_DONT_EXIST");

    match result {
        Err(Error::InvalidData(validate::Error::ProjectDoesNotExist)) => (),
        _ => panic!(result),
    }
}

#[test]
fn list_all() {
    let (client, _) = init_test_projects();
    let projects = Project::list_all(&client).unwrap();
    assert_eq!(NUM_PROJECTS, projects.len());

    for i in (0..NUM_PROJECTS) {
        let index = projects.iter().position(|project| project.name == NAMES[i]).unwrap();
        assert_eq!(GOALS[i], projects[index].goal);
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
    let (client, _) = init_test_projects();
    let _ = init_test_pledges(&client);

    // List backers.
    let (backers, goal) = Project::list_backers(&client, NAMES[0]).unwrap();
    assert_eq!(GOALS[0], goal);
    
    for (backer, pledge) in backers {
        let index = USERS.iter().position(|&name| name == backer.name).unwrap();
        assert_eq!(CONTRIBUTIONS[index], pledge.amount);
    }
}

#[test]
fn list_backers_none() {
    let (client, _) = init_test_projects();
    let (backers, goal) = Project::list_backers(&client, NAMES[0]).unwrap();
    assert_eq!(GOALS[0], goal);
    assert!(backers.is_empty());
}

#[test]
fn list_backers_missing() {
    let client = init_client();
    let result = Project::list_backers(&client, "PSYCHE");
    
    match result {
        Err(Error::InvalidData(validate::Error::ProjectDoesNotExist)) => (),
        _ => panic!(result),
    }
}
