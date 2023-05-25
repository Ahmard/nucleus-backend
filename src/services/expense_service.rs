use crate::models::expense::Expense;
use crate::models::DBPool;
use crate::repositories::expense_repository::ExpenseRepository;
use diesel::QueryResult;
use uuid::Uuid;

pub struct ExpenseService;

impl ExpenseService {
    pub fn create(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        project_id: Uuid,
        amount: i64,
        narration: String,
        spent_at: chrono::NaiveDateTime,
    ) -> Expense {
        ExpenseRepository.create(pool, user_id, project_id, amount, narration, spent_at)
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
