use crate::helpers::error_messages::db_failed_to_acquire_conn;
use crate::models::DBPool;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use r2d2::PooledConnection;

pub mod auth;
pub mod db;
pub mod error_messages;
pub mod http;
pub mod number;
pub mod responder;
pub mod string;
pub mod db_pagination;

pub fn get_db_conn(pool: &DBPool) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect(db_failed_to_acquire_conn())
}
