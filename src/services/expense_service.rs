use crate::models::expense::Expense;
use crate::models::DBPool;
use crate::repositories::expense_repository::ExpenseRepository;
use diesel::QueryResult;
use uuid::Uuid;
use crate::repositories::budget_repository::BudgetRepository;
use crate::services::budget_service::BudgetService;

pub struct ExpenseService;

impl ExpenseService {
    pub fn create<'a>(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        project_id: Uuid,
        amount: i64,
        narration: String,
        spent_at: chrono::NaiveDateTime,
    ) -> Result<Expense, &'a str> {
        let result = BudgetRepository.find_owned_current_month_budget(pool, user_id).unwrap();
        if result.is_none() {
            return Err("No budget for current month found");
        }

        let mut budget = result.unwrap();
        if amount > budget.available_amount() {
            return Err("This expense exceeds current budget");
        }

        let expense = ExpenseRepository.create(
            pool,
            user_id,
            project_id,
            budget.budget_id,
            amount,
            narration,
            spent_at,
        );

        BudgetService
            .decrement(pool, &mut budget, amount)
            .expect("Failed to decrement budget");

        Ok(expense)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        project_id: Uuid,
        amount: i64,
        narration: String,
        spent_at: chrono::NaiveDateTime,
    ) -> QueryResult<Expense> {
        ExpenseRepository.update(pool, id, user_id, project_id, amount, narration, spent_at)
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Expense> {
        ExpenseRepository.delete(pool, id, user_id)
    }
}
