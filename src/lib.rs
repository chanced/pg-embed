//!
//! # pg-embed
//!
//! [![Crates.io](https://img.shields.io/crates/v/pg-embed)](http://crates.io/crates/pg-embed)
//! [![Docs.rs](https://docs.rs/pg-embed/badge.svg)](https://docs.rs/pg-embed)
//! [![Crates.io](https://img.shields.io/crates/d/pg-embed)](http://crates.io/crates/pg-embed)
//! [![Crates.io](https://img.shields.io/crates/l/pg-embed)](https://github.com/faokunega/pg-embed/blob/master/LICENSE)
//!
//! Run a Postgresql database locally on Linux, MacOS or Windows as part of another Rust application or test.
//!
//! # Usage
//!
//! Import following packages:
//!
//! `pg-embed = "0.1"`
//!
//! `zip = "0.5.11"`
//!
//! A postgresql instance can be created using<br/>
//! [postgres::PgEmbed]::new([postgres::PgSettings], [fetch::FetchSettings]) <br/>
//!
//!
//! # Examples
//!
//! ```
//! use pg_embed::postgres::{PgEmbed, PgSettings};
//! use pg_embed::fetch;
//! use pg_embed::fetch::{OperationSystem, Architecture, FetchSettings, PG_V13};
//!
//! let pg_settings = PgSettings{
//!     executables_dir: "data/postgres".to_string(),
//!     database_dir: "data/db".to_string(),
//!     user: "postgres".to_string(),
//!     password: "password".to_string(),
//!     persistent: false
//! };
//! let fetch_settings = FetchSettings{
//!     host: "https://repo1.maven.org".to_string(),
//!     operating_system: OperationSystem::Darwin,
//!     architecture: Architecture::Amd64,
//!     version: PG_V13
//! };
//! let mut pg_emb = PgEmbed::new(pg_settings, fetch_settings);
//!
//! async {
//!     /// download postgresql
//!     pg_emb.aquire_postgres().await;
//!
//!     /// create database password file
//!     pg_emb.create_password_file().await;
//!
//!     /// initialize postgresql database
//!     pg_emb.init_db().await;
//!
//!     /// start postgresql database
//!     pg_emb.start_db().await;
//!
//!     /// stop postgresql database
//!     pg_emb.stop_db().await;
//! };
//!
//!
//! ```
//!
pub mod fetch;
pub mod postgres;
pub mod errors;

