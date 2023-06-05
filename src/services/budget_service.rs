use crate::core::enums::http_error::DBResult;
use crate::core::helpers::get_db_conn;
use crate::models::budget::{Budget, BudgetForm};
use crate::models::DBPool;
use crate::repositories::budget_repository::BudgetRepository;
use crate::schema::budgets;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use std::ops::DerefMut;
use uuid::Uuid;

pub struct BudgetService;

impl BudgetService {
    pub fn create(&mut self, pool: &DBPool, user_id: Uuid, form: BudgetForm) -> Budget {
        BudgetRepository.create(pool, user_id, form)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        form: BudgetForm,
    ) -> DBResult<Budget> {
        BudgetRepository.update(pool, id, user_id, form)
    }

    pub fn decrement(
        &mut self,
        pool: &DBPool,
        budget: &Budget,
        amount: i64,
    ) -> QueryResult<Budget> {
        let used_amount = budget.amount_used + amount;
        diesel::update(budgets::table)
            .filter(budgets::budget_id.eq(budget.budget_id))
            .set(budgets::amount_used.eq(used_amount))
            .get_result::<Budget>(get_db_conn(pool).deref_mut())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> DBResult<Budget> {
        BudgetRepository.delete(pool, id, user_id)
    }
}
