#![allow(clippy::extra_unused_lifetimes)]

use diesel::{Associations, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::super::schema::expenses;
use crate::models::project::Project;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Associations, Insertable, Queryable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(table_name = expenses)]
pub struct Expense {
    pub expense_id: String,
    pub project_id: String,
    pub user_id: String,
    pub amount: i64,
    pub narration: String,
    pub spent_at: chrono::NaiveDateTime,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct ExpenseForm {
    pub project_id: String,
    pub amount: i64,
    pub narration: String,
    pub spent_at: Option<String>,
}