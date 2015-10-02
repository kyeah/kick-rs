extern crate docopt;
extern crate rustc_serialize;
extern crate kickstarter;

use docopt::Docopt;
use kickstarter::{pledge, project, Client, Error};

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

    if args.flag_sync {
        client.sync();
    }

    if args.cmd_project {
        let name     = args.arg_name.unwrap();
        let amount   = args.arg_amount.unwrap();
        let project  = try_return!(client.create_project(&name, amount));

    } else if args.cmd_back {
        let user     = args.arg_user.unwrap();
        let name     = args.arg_name.unwrap();
        let card     = args.arg_card.unwrap();
        let amount   = args.arg_amount.unwrap();
        let project  = try_return!(client.back_project(&user, &name, &card, amount));

    } else if args.cmd_list {
        let name     = args.arg_name.unwrap();
        let backers  = try_return!(client.list_backers(&name));

    } else if args.cmd_backer {
        let user     = args.arg_user.unwrap();
        let projects = try_return!(client.list_backed_projects(&user));
    }
}
