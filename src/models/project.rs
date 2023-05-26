#![allow(clippy::extra_unused_lifetimes)]

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::projects;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = projects)]
pub struct Project {
    pub project_id: Uuid,
    pub user_id: Uuid,
    pub name: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectForm {
    pub name: String,
    pub description: String,
}
