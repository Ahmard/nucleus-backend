#![allow(clippy::extra_unused_lifetimes)]

use diesel::sql_types::{Nullable, VarChar};
use diesel::{Associations, Insertable, Queryable, QueryableByName};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::super::schema::expenses;
use crate::models::project::Project;
use crate::models::user::User;

#[derive(Debug, Serialize, Deserialize, Associations, Insertable, Queryable, Clone)]
#[diesel(belongs_to(User))]
#[diesel(belongs_to(Project))]
#[diesel(table_name = expenses)]
pub struct Expense {
    pub expense_id: Uuid,
    pub user_id: Uuid,
    pub project_id: Uuid,
    pub budget_id: Uuid,
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

#[derive(QueryableByName, Serialize)]
pub struct ExpenseAggregate {
    #[diesel(sql_type = Nullable<VarChar>)]
    pub year_expenses: Option<String>,
    #[diesel(sql_type = Nullable<VarChar>)]
    pub month_expenses: Option<String>,
    #[diesel(sql_type = Nullable<VarChar>)]
    pub week_expenses: Option<String>,
    #[diesel(sql_type = Nullable<VarChar>)]
    pub today_expenses: Option<String>,
}
