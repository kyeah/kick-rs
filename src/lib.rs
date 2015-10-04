//! # The Real Kickstarter
//!
//! A mini-kickstarter client.
//!
//! ## Usage
//!
//! The Client is our entry-point to interacting with our Kickstarter application.
//!
//! ```no_run
//! extern crate kickstarter;
//! use kickstarter::Client;

//! fn main() {
//!     let client = Client::with_config("postgres://user:pass@localhost:5432/kickstarter", 
//!                                      "kickstarter").unwrap();
//!
//!     let project = client.create_project("Meditation_Witchcraft", 520.25f64).unwrap();
//!     println!("Created project {}!", project.name);
//!
//!     let pledge = client.back_project("JHernandez", "Meditation_Witchcraft", 10f64).unwrap();
//!     println!("Backed for ${:.2}!", pledge.amount);
//!
//!     let (backers, goal) = client.list_backers("Meditation_Witchcraft").unwrap();
//!     for (backer, contribution) in backers {
//!         println!("{} backed for ${:.2}.", backer.name, contribution);
//!     }
//!     
//!     let results = client.backed_projects("JHernandez").unwrap();
//!     for (project_name, pledge) in results {
//!         println!("JHernandez backed {} for ${:.2}.", project_name, pledge.amount);
//!     }
//!
//!     let all_projects = client.list_projects().unwrap();
//!     for project in all_projects {
//!         println!("{} is raising ${:.2}.", project.name, project.goal);
//!     }
//! }
//! ```

#[macro_use]
extern crate lazy_static;

extern crate chrono;
extern crate codegenta;
extern crate postgres;
extern crate regex;
extern crate rustc_serialize;
extern crate rustorm;
extern crate toml;

pub mod db;
pub mod pledge;
pub mod project;
pub mod user;
pub mod validate;

mod client;
mod error;

pub use db::kickstarter as models;
pub use client::Client;
pub use error::{Error, Result};
