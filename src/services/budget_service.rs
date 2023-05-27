use crate::models::budget::Budget;
use crate::models::DBPool;
use crate::repositories::budget_repository::BudgetRepository;
use diesel::QueryResult;
use uuid::Uuid;

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

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Budget> {
        BudgetRepository.delete(pool, id, user_id)
    }
}
