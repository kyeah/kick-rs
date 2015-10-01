extern crate codegenta;
extern crate docopt;
extern crate toml;
extern crate rustc_serialize;
extern crate rustorm;

extern crate kickstarter;

use docopt::Docopt;

use codegenta::generator::{self, Config};
use rustorm::pool::ManagedPool;

use std::fs::File;
use std::io::Read;

use kickstarter::{pledge, project, Error};

macro_rules! version {
    () => {
        format!("{}.{}.{}{}",
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH"),
                option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    }
}

const ERR_MISSING_CONFIG: &'static str = "Missing data/config.toml!";
const ERR_READING_CONFIG: &'static str = "Failed to read configuration into String!";
const ERR_PARSING_CONFIG: &'static str = "Failed to parse configuration file";
const ERR_MISSING_URI: &'static str = "Configuration has no database connection string 'db.uri'";

const USAGE: &'static str = "
The Real Kickstarter.

Usage:
    ksr project <name> <amount>
    ksr back    <user> <name> <card> <amount>
    ksr list    <name>
    ksr backer  <user>
    ksr (-h | --help)
    ksr (-s | --sync)
    ksr (-v | --version)

Options:
    -h --help      Show this message
    -v --version   Show version
    -s --sync      Sync generated models with db tables

Commands:
    project    Create a new project
    back       Back a project
    list       List all pledges towards a project
    backer     List all pledges that a backer has made

Examples:
    project Sensel_Control_Pad 250000.00
      $ Added Sensel_Control_Pad project with target of $250,000.00

    back Jorge Sensel_Control_Pad 123456789012 300
      $ Jorge backed project Sensel_Control_Pad for $300.00

    list Sensel_Control_Pad
      $ -- Jorge backed for $300.00
      $ Sensel_Control_Pad needs $249,700.00 more dollars to be successful

    backer Jorge
      $ Jorge backed project Sensel_Control_Pad for $300.00
";

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_project: bool,
    cmd_back: bool,
    cmd_list: bool,
    cmd_backer: bool,
    arg_user: Option<String>,
    arg_name: Option<String>,
    arg_card: Option<String>,
    arg_amount: Option<f64>,
    flag_version: bool,
    flag_sync: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());
    
    if args.flag_version {
        println!("{}", version!());
        return;
    }

    // Open config file
    let mut f = File::open("data/config.toml").ok().expect(ERR_MISSING_CONFIG);

    let mut toml = String::new();
    f.read_to_string(&mut toml).ok().expect(ERR_READING_CONFIG);

    let mut parser = toml::Parser::new(&mut toml);
    let config = parser.parse().expect(&format!("{}: {:?}", ERR_PARSING_CONFIG, parser.errors));

    // Retrieve and open database connection uri
    let uri = match config.get("uri") {
        Some(ref uri) => uri.as_str().unwrap(),
        None => panic!(ERR_MISSING_URI),
    };

    let pool = ManagedPool::init(uri, 1).unwrap();
    let db = pool.connect().unwrap();

    // Sync generated models in src/lib/gen with database tables.
    if args.flag_sync {
        let config = Config {
            base_module: Some("gen".to_string()),
            include_table_references: true,
            use_condensed_name: true,
            generate_table_meta: true,
            base_dir: "./src".to_string(),
            include_views: true,
        };

        generator::generate_all(db.as_dev(), &config);
    }

    if args.cmd_project {
        let project = project::create(args.arg_name.unwrap(), 
                                      args.arg_amount.unwrap())
            .unwrap();

    } else if args.cmd_back {
        let project = pledge::create(args.arg_user.unwrap(), args.arg_name.unwrap(), 
                                     args.arg_card.unwrap(), args.arg_amount.unwrap())
            .unwrap();

    } else if args.cmd_list {
        let backers = project::list_backers(args.arg_name.unwrap()).unwrap();

    } else if args.cmd_backer {
        let projects = pledge::list_backed(args.arg_user.unwrap()).unwrap();
    }
}
