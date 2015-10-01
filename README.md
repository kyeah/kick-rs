The Real Kickstarter
=====================

## Getting Started

### Database Setup

This project uses PostgreSQL for persistent storage. Before running ksr, ensure that you have PostgreSQL setup and running. On Ubuntu machines, you can run `sudo apt-get install postgresql postgresql-contrib` and [follow this guide](https://help.ubuntu.com/lts/serverguide/postgresql.html) to set it up.

To create and prepare the database from scratch, simply run the following commands:

* Create the db: `createdb kickstarter`
* Connect to the database: `psql -U postgres kickstarter` or `psql kickstarter`
* Create the tables: `\i data/tables.sql`
* Rename `sample-config.toml` to `config.toml` and point it to the database using a connection string: `postgres://<user>:<pass>@<ip>:<port>/kickstarter`. The default IP and port is localhost:5432.