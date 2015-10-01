extern crate rustc_serialize;
extern crate docopt;

use std::error::Error;
use docopt::Docopt;

macro_rules! version {
    () => {
        format!("{}.{}.{}{}",
                env!("CARGO_PKG_VERSION_MAJOR"),
                env!("CARGO_PKG_VERSION_MINOR"),
                env!("CARGO_PKG_VERSION_PATCH"),
                option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    }
}

const USAGE: &'static str = "
The Real Kickstarter.

Usage:
    ksr project <name> <amount>
    ksr back    <user> <name> <amount>
    ksr list    <name>
    ksr backer  <user>
    ksr (-h | --help)
    ksr (-v | --version)

Options:
    -h --help      Show this message
    -v --version   Show version

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
    arg_amount: Option<f64>,
    flag_version: bool,
}

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    if args.flag_version {
        println!("{}", version!());
        return;
    }

    if args.cmd_project {
        
    }

    if args.cmd_back {
        
    }

    if args.cmd_list {
        
    }

    if args.cmd_backer {
        
    }
}
