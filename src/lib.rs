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
//!
//! fn main() {
//!     let client = Client::new("postgres://user:pass@localhost:5432/kickstarter", 
//!                              "kickstarter").unwrap();
//!
//!     let project = client.create_project("Meditation_Witchcraft", 520.25f64).unwrap();
//!     println!("Created project {}!", project.name);
//!
//!     let pledge = client.back_project("JHernandez", "Meditation_Witchcraft", 
//!                                      "4298708533045499", 10f64).unwrap();
//!
//!     println!("Backed for ${:.2}!", pledge.amount);
//!
//!     let (pledges, goal) = client.list_backers("Meditation_Witchcraft").unwrap();
//!     for pledge in &pledges {
//!         let backer = pledge.get_user();
//!         println!("{} backed for ${:.2}.", backer.name, pledge.amount);
//!     }
//!     
//!     let pledges = client.list_backed_projects("JHernandez").unwrap();
//!     for pledge in &pledges {
//!         let project = pledge.get_project();
//!         println!("JHernandez backed {} for ${:.2}.", project.name, pledge.amount);
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
