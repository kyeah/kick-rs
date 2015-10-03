extern crate docopt;
extern crate rustc_serialize;
extern crate kickstarter;

use docopt::Docopt;
use kickstarter::{Client, Error};

const USAGE: &'static str = "
The Real Kickstarter.

Usage:
    ksr project <name> <amount>
    ksr back    <user> <name> <card> <amount>
    ksr list    <name>        
    ksr backer  <user>
    ksr listall
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

macro_rules! version {
    () => {
        format!("{}.{}.{}{}",
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH"),
                option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    }
}

macro_rules! try_return {
    ($expr:expr) => (match $expr {
        std::result::Result::Ok(val) => val,
        std::result::Result::Err(err) => { println!("ERROR: {}", err); return; },
    })
}

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_project: bool,
    cmd_back: bool,
    cmd_list: bool,
    cmd_backer: bool,
    cmd_listall: bool,
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

    let client = Client::with_config("data/config.toml").unwrap();

    // Wipe the database the sync it with the configuration file, then generate the associated models.
    if args.flag_sync {
        client.sync();
    }

    if args.cmd_project {
        let name    = args.arg_name.unwrap();
        let amount  = args.arg_amount.unwrap();
        try_return!(client.create_project(&name, amount));
        println!("Added project '{}' with a target goal of ${:.2}.", name, amount);

    } else if args.cmd_back {
        let user    = args.arg_user.unwrap();
        let name    = args.arg_name.unwrap();
        let card    = args.arg_card.unwrap();
        let amount  = args.arg_amount.unwrap();
        try_return!(client.back_project(&user, &name, &card, amount));
        println!("{} backed project '{}' for ${:.2}.", user, name, amount);

    } else if args.cmd_list {
        let name    = args.arg_name.unwrap();
        let (results, goal) = try_return!(client.list_backers(&name));

        if results.is_empty() {
            println!("{} doesn't have any backers yet. Maybe you'd like to help it get off the ground?", name);
        } else {
            let mut total = 0f64;
            for (user, &amount) in &results {
                println!("-- {} backed for ${:.2}", user.name, amount);
                total += amount;
            }

            if total < goal {
                println!("{} needs ${:.2} more dollars to be successful!", name, goal - total);
            } else {
                println!("{} is successfully funded!", name);
            }
        }

    } else if args.cmd_backer {
        let user    = args.arg_user.unwrap();
        let results = try_return!(client.list_backed_projects(&user));

        if results.is_empty() {
            println!("{} hasn't backed any projects...yet. Get to it!", user);
        } else {
            let mut total = 0f64;
            for (project, pledge) in &results {
                println!("{} backed project {} for ${:.2}", user, project, pledge.amount);
                total += pledge.amount;
            }
            println!("{} has given ${:.2} back to their community. Thanks {}!", user, total, user);
        }

    } else if args.cmd_listall {
        let projects = try_return!(client.list_projects());
        if projects.is_empty() {
            println!("There aren't any projects on Kickstarter right now. Check again in a little while!");
        } else {
            for project in projects {
                println!("Project '{}' is raising ${:.2}", project.name, project.goal);
            }
        }
    }
}
