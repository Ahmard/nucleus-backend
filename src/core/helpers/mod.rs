use crate::models::DBPool;
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub mod auth;
pub mod date_time;
pub mod db;
pub mod db_pagination;
pub mod form;
pub mod http;
pub mod misc;
pub mod number;
pub mod responder;
pub mod string;

pub fn get_db_conn(pool: &DBPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get()
        .unwrap_or_else(|_| panic!("Failed to acquire database connection from connection pools"))
}
