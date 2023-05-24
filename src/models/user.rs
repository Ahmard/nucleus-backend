#![allow(clippy::extra_unused_lifetimes)]

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::schema::users;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = users)]
pub struct User {
    pub user_id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub status: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub enum UserStatus {
    ACTIVE,
    INACTIVE,
    PENDING,
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct RegisterForm {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}
