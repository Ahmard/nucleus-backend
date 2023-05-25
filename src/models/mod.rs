#![allow(clippy::extra_unused_lifetimes)]

pub mod expense;
pub mod label;
pub mod project;
pub mod user;

use diesel::{r2d2::ConnectionManager, PgConnection};

// type alias to use in multiple places
pub type DBPool = r2d2::Pool<ConnectionManager<PgConnection>>;
