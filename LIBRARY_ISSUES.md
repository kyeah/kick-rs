Bugs
=====

Over the course of building this tool, I ran into a number of bugs and missing features in the latest Rust release and third-party ORMs. Below is an incomplete list of issues that were fixed on local library forks or had to be worked around due to time constraints.

## Rustorm & Codegenta

### Fixed
* Duplicated columns due to CHECK constraints were not unified in the PostgreSQL platform, leading to inaccurate column counts and information, and uncompilable generated models from Codegenta.
* Rustorm has a very...hashed-together error system, that doesn't propogate errors up from the libraries that it uses. The error system needs to be idiomaticized as an enum with `Error` wrappers and `From<>` traits, but for now, I manually pull important description and error code information from PostgreSQL database errors, allowing me to handle different database errors accordingly (such as UniqueViolation). This also propagates the actual descriptions instead of defaulting to "Something went wrong", which is an extraordinarily disheartening and unhelpful response.
* Some query string builds were badly formatted, leading to invalid sql executions. Ex. `SELECT * FROM kickstarter.pledge plINNER JOIN ...`
* Timestamps were not parsed properly into DateTime, leading to NULL values and invalid models that failed silently and were left out of certain queries.

### Unfixed
* No support for custom DOMAIN types. This prevented the use of DOMAIN types like alphanum and numtext to avoid extraneous CHECK constraints on each column that needed it.
* No support for arbitrary-precision types like numeric. This made it impossible to offload currency rounding to the database.
* Generated models don't derive `PartialEq` and `Eq`.

## Rust-lang
* The Rust language has severe floating point parse inaccuracies that are fixed in v1.4, coming out in a few weeks. This drove me crazy, since values like 0.12 would be parsed as .120000000001. As a result, you may notice some funny accuracy issues with pledge amounts and goal markers.
