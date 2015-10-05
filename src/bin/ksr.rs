extern crate docopt;
extern crate rustc_serialize;
extern crate kickstarter;

use docopt::Docopt;
use kickstarter::{Client, Result, Error};
use std::fs::File;
use std::io::{self, BufRead, Read, Write};

const SEPARATOR: &'static str = "=======================================";

const USAGE: &'static str = "
The Real Kickstarter.

Usage:
    ksr run     [<file>]                       [--config=<cfile>]
    ksr project <name> <amount>                [--config=<cfile>]
    ksr back    <user> <name> <card> <amount>  [--config=<cfile>]
    ksr list    <name>                         [--config=<cfile>]
    ksr backer  <user>                         [--config=<cfile>]
    ksr listall                                [--config=<cfile>]
    ksr (-b | --build)                         [--config=<cfile>]
    ksr (-s | --sync)                          [--config=<cfile>]
    ksr (-h | --help)
    ksr (-v | --version)

Options:
    -h --help          Show this message
    -v --version       Show version
    -s --sync          Sync generated models with db tables
    -b --build         Build tables and models from configured .sql file
    --config=<cfile>   The .toml configuration file [default: data/config.toml]

Commands:
    project    Create a new project
    back       Back a project
    list       List all pledges towards a project
    backer     List all pledges that a backer has made
    listall    List all existing projects
    run        Streaming CLI

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

/// try! macro, but print the error description and return void.
macro_rules! try_return {
    ($expr:expr) => (match $expr {
        std::result::Result::Ok(val) => val,
        std::result::Result::Err(err) => { println!("ERROR: {}", err); return; },
    })
}

#[derive(Debug, RustcDecodable)]
struct Args {
    cmd_run: bool,
    cmd_project: bool,
    cmd_back: bool,
    cmd_list: bool,
    cmd_backer: bool,
    cmd_listall: bool,
    arg_file: Option<String>,
    arg_user: Option<String>,
    arg_name: Option<String>,
    arg_card: Option<String>,
    arg_amount: Option<f64>,
    flag_version: bool,
    flag_sync: bool,
    flag_build: bool,
    flag_config: String,
}

fn main() {

    let docopt = try_return!(Docopt::new(USAGE));

    let args: Args = docopt.clone().decode()
        .unwrap_or_else(|e| e.exit());

    // Print the library version.
    if args.flag_version {
        println!("{}", version!());
        return;
    }

    // Destructive flag -- double-check with the user.
    if args.flag_build {
        try_return!(ensure_build());
    }

    // Connect to the database.
    let client = match Client::with_config(&args.flag_config, args.flag_build, true) {
        Ok(client) => client,
        Err(ref e) => {
            println!("ERROR: {} \n{}\n\
                      Could not connect to the database. \n\
                      Make sure that `{}` exists and is pointing to an existing database.",
                     e, SEPARATOR, &args.flag_config);
            return;
        }
    };

    // Wipe the database and sync it with the configuration file, then generate the associated models.
    if args.flag_sync {
        client.sync();
    }

    if args.flag_build || args.flag_sync {
        return;
    }
    
    // Handle main kickstarter commands.
    if !args.cmd_run {
        handle_args(&client, args);
    } else {
        match args.arg_file {
            None => prompt(client, docopt),
            Some(ref filename) => run_file(client, docopt, filename),
        }
    }
}

/// Ensure that the user wishes to rebuild the database and models.
fn ensure_build() -> Result<()> {
    // Print prompt and flush to display
    print!("WARNING: The --build flag will destroy and rebuild the database. \n\
            Are you sure you want to continue? [Y/N]: ");

    try!(io::stdout().flush());

    // Read in user input
    let mut ans = String::new();
    try!(io::stdin().read_line(&mut ans));

    if !ans.starts_with("y") && !ans.starts_with("Y") {
        Err(Error::Config("Cancelled rebuild.".to_owned()))
    } else {
        Ok(())
    }
}

// Run commands from stdin.
fn prompt(client: Client, docopt: Docopt) {
    let stdin = io::stdin();
    print!("> ");
    try_return!(io::stdout().flush());

    for line in stdin.lock().lines() {
        let args = read_args(docopt.clone(), &line.unwrap());
        handle_args(&client, args);

        println!("{}", SEPARATOR);

        print!("> ");
        try_return!(io::stdout().flush());
    }
}

// Run a list of commands provided via file.
// This will not run anything unless all lines are successfully parsed.
fn run_file(client: Client, docopt: Docopt, filename: &str) {

    let cmds = try_return!(read_file_as_string(filename));

    // Split into a list of commands.
    let mut run_list: Vec<(&str, Args)> = vec![];
    let runs: Vec<&str> = cmds.split("\n")
        .filter(|s| !s.is_empty())
        .collect();

    // Parse each command.
    for run in &runs {
        let doc_args = read_args(docopt.clone(), run);
        run_list.push((run, doc_args));
    }

    // Run each command without checking for errors.
    for (cmd, args) in run_list {
        println!("> {}", cmd);
        handle_args(&client, args);
        println!("{}", SEPARATOR);
    }
}

fn read_file_as_string(filename: &str) -> Result<String> {
    let mut f = try!(File::open(filename));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

// Parses a single command without the `ksr` prefix.
fn read_args(docopt: Docopt, line: &str) -> Args {
    let formatted = format!("ksr {}", line);
    let args = formatted.split(" ");
    let doc = docopt.argv(args);
    doc.decode().unwrap_or_else(|e| {
        println!("Attempted to parse args: {}", line); 
        e.exit()
    })
}

// Run the appropriate command based on arguments.
fn handle_args(client: &Client, args: Args) {
    if args.cmd_project {
        cmd_project(&client, args);

    } else if args.cmd_back {
        cmd_back(&client, args);

    } else if args.cmd_list {
        cmd_list(&client, args);

    } else if args.cmd_backer {
        cmd_backer(&client, args);

    } else if args.cmd_listall {
        cmd_listall(&client);
    }
}

/// Create a new project with the desired amount.
fn cmd_project(client: &Client, args: Args) {
    let name    = args.arg_name.unwrap();
    let amount  = args.arg_amount.unwrap();
    try_return!(client.create_project(&name, amount));
    println!("Added project '{}' with a target goal of ${:.2}.", name, amount);
}

/// Back an existing project with a username, credit card, and contribution amount.
fn cmd_back(client: &Client, args: Args) {
    let user    = args.arg_user.unwrap();
    let name    = args.arg_name.unwrap();
    let card    = args.arg_card.unwrap();
    let amount  = args.arg_amount.unwrap();
    try_return!(client.back_project(&user, &name, &card, amount));
    println!("{} backed project '{}' for ${:.2}.", user, name, amount);
}

/// List all backers for an existing project.
fn cmd_list(client: &Client, args: Args) {
    let name    = args.arg_name.unwrap();
    let (results, goal) = try_return!(client.list_backers(&name));

    if results.is_empty() {
        println!("{} doesn't have any backers yet. Maybe you'd like to help it get off the ground?", name);
    } else {
        let mut total = 0f64;
        for (user, pledge) in &results {
            println!("-- {} backed for ${:.2}", user.name, pledge.amount);
            total += pledge.amount;
        }

        if total < goal {
            println!("{} needs ${:.2} more dollars to be successful!", name, goal - total);
        } else {
            println!("{} is successfully funded!", name);
        }
    }
}

/// List all projects that have been backed by a user.
fn cmd_backer(client: &Client, args: Args) {
    let user    = args.arg_user.unwrap();
    let results = try_return!(client.list_backed_projects(&user));

    if results.is_empty() {
        println!("{} hasn't backed any projects...yet. Get to it!", user);
    } else {
        let mut total = 0f64;
        for (project, pledge) in &results {
            println!("{} backed project '{}' for ${:.2}", user, project.name, pledge.amount);
            total += pledge.amount;
        }
        println!("{} has given ${:.2} back to their community. Thanks {}!", user, total, user);
    }
}

/// List all projects on Kickstarter.
fn cmd_listall(client: &Client) {
    let projects = try_return!(client.list_projects());
    if projects.is_empty() {
        println!("There aren't any projects on Kickstarter right now. Check again in a little while!");
    } else {
        for project in projects {
            println!("Project '{}' is raising ${:.2}", project.name, project.goal);
        }
    }
}
