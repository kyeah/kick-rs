extern crate kickstarter;
extern crate postgres;
extern crate rustorm;

mod models;

use kickstarter::Client;
use kickstarter::models::{Pledge, Project};

const DEFAULT_CONFIG: &'static str = "tests/data/config.toml";

// Test Projects
const NUM_PROJECTS: usize = 4;
static NAMES: &'static [&'static str] = &["GoGo_Applesauce", "Exquisite_Banana", 
                                          "Seattle_Dance_Party", "Alt_Party_Owl"];

static GOALS: &'static [f64] = &[250000f64, 1f64, 12500f64, 500f64];

// Test Pledges for GoGo Applesauce :)
const NUM_PLEDGES: usize = 3;
static USERS: &'static [&'static str] = &["Johnnyboy", "Margie", "Shakey_Graves"];
static CARDS: &'static [&'static str] = &["341468752760899",
                                          "351149395124027",
                                          "6011168468345649"];

static CONTRIBUTIONS: &'static [f64] = &[100f64, 200f64, 3000f64];

pub fn init_client() -> Client {
    Client::with_config(DEFAULT_CONFIG, true, false).unwrap_or_else(|e| {
        panic!("ERROR: {}\n\
                Could not connect to the test database. \
                Make sure that '{}' exists and is pointing to an existing database.", 
               e, DEFAULT_CONFIG);
    })
}

// Returns the client and a list of created projects.
fn init_test_projects() -> (Client, Vec<Project>) {
    let client = init_client();

    let mut projects = vec![];
    for i in (0..NUM_PROJECTS) {
        let project = Project::create(&client, NAMES[i], GOALS[i]).unwrap();
        projects.push(project);
    }

    (client, projects)
}

// Returns the list of created pledges.
fn init_test_pledges(client: &Client) -> Vec<Pledge> {
    let mut pledges = vec![];
    for i in (0..NUM_PLEDGES) {
        let pledge = Pledge::create(&client, USERS[i], NAMES[0], CARDS[i], CONTRIBUTIONS[i]).unwrap();
        pledges.push(pledge);
    }

    pledges
}
