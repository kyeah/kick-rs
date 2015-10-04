extern crate kickstarter;
extern crate postgres;
extern crate rustorm;

mod models;

use kickstarter::Client;

const DEFAULT_CONFIG: &'static str = "tests/data/config.toml";

pub fn init_client() -> Client {
    Client::with_config(DEFAULT_CONFIG, true, false).unwrap()
}
