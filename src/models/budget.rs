#![allow(clippy::extra_unused_lifetimes)]

use diesel::{Insertable, Queryable};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::budgets;

#[derive(Debug, Serialize, Deserialize, Insertable, Queryable, Clone)]
#[diesel(table_name = budgets)]
pub struct Budget {
    pub budget_id: Uuid,
    pub user_id: Uuid,
    pub amount: i64,
    pub amount_used: i64,
    pub month: i16,
    pub year: i16,
    pub title: String,
    pub comment: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

#[derive(Serialize, Deserialize)]
pub struct BudgetForm {
    pub amount: i64,
    pub month: i16,
    pub year: i16,
    pub comment: Option<String>,
}
