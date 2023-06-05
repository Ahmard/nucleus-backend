use crate::core::enums::http_error::DBResult;
use crate::models::expense::{Expense, ExpenseForm};
use crate::models::DBPool;
use crate::repositories::budget_repository::BudgetRepository;
use crate::repositories::expense_repository::ExpenseRepository;
use crate::services::budget_service::BudgetService;
use uuid::Uuid;

pub struct ExpenseService;

impl ExpenseService {
    pub fn create<'a>(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        form: ExpenseForm,
    ) -> Result<Expense, &'a str> {
        let result = BudgetRepository
            .find_owned_current_month_budget(pool, user_id)
            .unwrap();
        if result.is_none() {
            return Err("No budget for current month found");
        }

        let amount = form.amount;
        let mut budget = result.unwrap();
        if amount > budget.available_amount() {
            return Err("This expense exceeds current budget");
        }

        let expense = ExpenseRepository.create(pool, user_id, budget.budget_id, form);

        BudgetService
            .decrement(pool, &budget, amount)
            .expect("Failed to decrement budget");

        Ok(expense)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        form: ExpenseForm,
    ) -> DBResult<Expense> {
        ExpenseRepository.update(pool, id, user_id, form)
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> DBResult<Expense> {
        ExpenseRepository.delete(pool, id, user_id)
    }
}
