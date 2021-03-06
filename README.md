The Real Kickstarter
=====================

A mini-Kickstarter application and CLI playground for identifying issues in current Rust ORMs and command-line parsers. Also James told me to make this. Hi James!

[See documentation for a high-level library overview.](https://kyeah.github.io/kick-rs/kickstarter)

```c
$ ksr run
> project Sensel_Control_Pad 250000.00
Added project 'Sensel_Control_Pad' with a target goal of $250,000.00.

> back Sally Sensel_Control_Pad 4773718568425957 300
Sally backed project 'Sensel_Control_Pad' for $300.00.

> list Sensel_Control_Pad
-- Sally backed for $300.00
Sensel_Control_Pad needs $249,700.00 more dollars to be successful!

> backer Sally
Sally backed project 'Sensel_Control_Pad' for $300.00
Sally has given $300.00 back to their community. Thanks Sally!
```

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

### As a Library

The Kickstarter client library is fully compatible with Cargo, and is available as a git dependency. Just add it to your `Cargo.toml`:

```toml
[dependencies.kickstarter]
git = "https://github.com/kyeah/kick-rs"
```

## Getting Started

### Prerequisites

To build this project, you will need [Rust 1.0+](https://www.rust-lang.org) and its package manager, Cargo. The easiest way to get the current stable release of Rust and Cargo is using `rustup`:

```sh
$ curl -sSf https://static.rust-lang.org/rustup.sh | sh
```

This project also uses PostgreSQL for persistent storage. Before running ksr, ensure that you have PostgreSQL setup and running. 

```sh
$ sudo apt-get install postgresql postgresql-contrib
```

You can [follow this guide](https://help.ubuntu.com/lts/serverguide/postgresql.html) to set up your service and user permissions.

### Building ksr

Thanks to Cargo, building Rust packages is really easy! 

```sh
$ git clone https://github.com/kyeah/kick-rs && cd kick-rs
$ cargo build [--release]
```

The `ksr` binary executable will be built under `target/debug` or `target/release`.

### Database Setup

```sh
$ createdb <db_name> (default: kickstarter)
$ cp data/sample-config.toml data/config.toml
```

You'll need to change the connection string in `config.toml` to point to your database.

```toml
uri = "postgres://<user>:<pass>@<ip>:<port>/kickstarter"
```

Then build or rebuild the schema.

```sh
$ ksr --build
```

This will bootstrap the database and regenerate the models in [src/db](src/db). Pretty cool!

### Testing

The integration tests rely on a live test database defined in [tests/data](tests/data). Make sure that the provided database exists and that the Rust tests are running on a single-thread before executing `cargo test`.

```sh
$ createdb ksr-test
$ cp tests/data/sample-config.toml tests/data/config.toml
$ export RUST_TEST_THREADS=1
$ cargo test
```

Don't forget to modify `config.toml` to provide database credentials!