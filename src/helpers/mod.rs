use crate::helpers::error_messages::db_failed_to_acquire_conn;
use crate::models::DBPool;
use diesel::r2d2::ConnectionManager;
#[allow(dead_code)]
use diesel::MysqlConnection;
use r2d2::PooledConnection;

pub mod auth;
pub mod db;
pub mod error_messages;
pub mod http;
pub mod number;
pub mod responder;
pub mod string;

pub fn get_db_conn(pool: &DBPool) -> PooledConnection<ConnectionManager<MysqlConnection>> {
    let conn = pool.get().expect(db_failed_to_acquire_conn());
    return conn;
}
