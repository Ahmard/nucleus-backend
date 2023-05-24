#![allow(clippy::extra_unused_lifetimes)]

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::schema::labels;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = labels)]
pub struct Label {
    pub label_id: String,
    pub name: String,
    pub module: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct LabelForm {
    pub name: String,
    pub module: String,
}
