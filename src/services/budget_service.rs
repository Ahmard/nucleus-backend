use std::ops::DerefMut;
use crate::models::budget::Budget;
use crate::models::DBPool;
use crate::repositories::budget_repository::BudgetRepository;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use uuid::Uuid;
use crate::helpers::get_db_conn;
use crate::schema::budgets;

pub struct BudgetService;

impl BudgetService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        amount: i64,
        month: i16,
        year: i16,
        comment: Option<String>,
    ) -> Budget {
        BudgetRepository.create(pool, user_id, amount, month, year, comment)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        amount: i64,
        month: i16,
        year: i16,
        comment: Option<String>,
    ) -> QueryResult<Budget> {
        BudgetRepository.update(pool, id, user_id, amount, month, year, comment)
    }

    pub fn decrement(&mut self, pool: &DBPool, budget: &Budget, amount: i64) -> QueryResult<Budget> {
        let used_amount = budget.amount_used + amount;
        diesel::update(budgets::table)
            .filter(budgets::budget_id.eq(budget.budget_id))
            .set(budgets::amount_used.eq(used_amount))
            .get_result::<Budget>(get_db_conn(pool).deref_mut())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Budget> {
        BudgetRepository.delete(pool, id, user_id)
    }
}
