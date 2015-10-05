The Real Kickstarter
=====================

A mini-Kickstarter application and CLI playground for identifying issues in current Rust ORMs and command-line parsers. Also James told me to make this. Hi James!

[See documentation for a high-level library overview.](https://kyeah.github.io/kick-rs/kickstarter)

## Usage

### Command-line Interface

A simple CLI is provided for bootstrapping and interacting with a local copy of (the real) Kickstarter.

```sh
Usage:
    ksr run     [<file>]
    ksr project <name> <amount>
    ksr back    <user> <name> <card> <amount>
    ksr list    <name>
    ksr backer  <user>
    ksr listall
    ksr (-h | --help)
    ksr (-v | --version)
    ksr (-b | --build)
    ksr (-s | --sync)

Options:
    -h --help      Show this message
    -v --version   Show version
    -s --sync      Sync generated models with db tables
    -b --build     Build tables and models from configured .sql file
    --config=<f>   The .toml configuration file [default: data/config.toml]

Commands:
    project    Create a new project
    back       Back a project
    list       List all pledges towards a project
    backer     List all pledges that a backer has made
    listall    List all existing projects
    run        Streaming CLI
```

Here are a few examples of usage:
```
project Sensel_Control_Pad 250000.00
  $ Added Sensel_Control_Pad project with target of $250,000.00

back Jorge Sensel_Control_Pad 123456789012 300
  $ Jorge backed project Sensel_Control_Pad for $300.00

list Sensel_Control_Pad
  $ -- Jorge backed for $300.00
  $ Sensel_Control_Pad needs $249,700.00 more dollars to be successful

backer Jorge
  $ Jorge backed project Sensel_Control_Pad for $300.00
```

### As a Library

The Kickstarter client library is fully compatible with Cargo, and is available as a git dependency. Just add it to your `Cargo.toml`:

```toml
[dependencies.kickstarter]
git = "https://github.com/kyeah/kick-rs"
```

## Getting Started

### Prerequisites

To build this project, you will need [Rust 1.0+](https://www.rust-lang.org) and its package manager, Cargo. The easiest way to get the current stable release of Rust and Cargo is using `rustup`:

`curl -sSf https://static.rust-lang.org/rustup.sh | sh`

This project also uses PostgreSQL for persistent storage. Before running ksr, ensure that you have PostgreSQL setup and running. 

`sudo apt-get install postgresql postgresql-contrib`

You can [follow this guide](https://help.ubuntu.com/lts/serverguide/postgresql.html) to set up your service and user permissions.

### Building ksr

Thanks to Cargo, building Rust packages is really easy! 

`cargo build [--release]`

The `ksr` binary executable will be built under `target/debug` or `target/release`.

### Database Setup

To create the database, run `createdb <db_name>`. The default name is `kickstarter`. 

Then, configure the application to connect to your database by renaming `data/sample-config.toml` to `data/config.toml` and changing the connection string.

`postgres://<user>:<pass>@<ip>:<port>/kickstarter`

The default IP and port is localhost:5432.

You can build or rebuild the schema at any time by running `ksr --build`. This will bootstrap the database and regenerate the models in [src/db](src/db). Pretty cool!

### Testing

The integration tests rely on a live test database defined in [tests/data](tests/data). Make sure that the provided database exists and that the Rust tests are running on a single-thread before executing `cargo test`.

`export RUST_TEST_THREADS=1`.