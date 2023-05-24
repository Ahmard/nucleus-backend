#![allow(clippy::extra_unused_lifetimes)]

mod label;
pub mod user;

use diesel::{r2d2::ConnectionManager, MysqlConnection};

// type alias to use in multiple places
pub type DBPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
