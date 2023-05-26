use crate::helpers::db::current_timestamp;
use crate::helpers::error_messages::db_failed_to_execute;
use crate::helpers::get_db_conn;
use crate::helpers::http::QueryParams;
use crate::helpers::number::to_cent;
use crate::models::expense::Expense;
use crate::models::project::Project;
use crate::models::DBPool;
use crate::schema::expenses;
use crate::schema::projects;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};
use std::ops::DerefMut;
use uuid::Uuid;

pub struct ExpenseRepository;

impl ExpenseRepository {
    pub fn list_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        mut query_params: QueryParams,
    ) -> QueryResult<Vec<(Expense, Project)>> {
        let builder = expenses::table
            .inner_join(projects::table)
            .filter(expenses::user_id.eq(id))
            .filter(expenses::deleted_at.is_null())
            .order_by(expenses::created_at.desc())
            .limit(query_params.get_limit());

        let search_format = format!("%{}%", query_params.get_search_query());

        builder
            .filter(expenses::narration.like(search_format))
            .get_results::<(Expense, Project)>(get_db_conn(pool).deref_mut())
    }

    pub fn list_by_project_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        mut query_params: QueryParams,
    ) -> QueryResult<Vec<(Expense, Project)>> {
        let mut conn = get_db_conn(pool);
        let builder = expenses::table
            .inner_join(projects::table)
            .filter(expenses::project_id.eq(id))
            .filter(expenses::deleted_at.is_null())
            .order_by(expenses::created_at.desc())
            .limit(query_params.get_limit());

        let search_format = format!("%{}%", query_params.get_search_query());
        builder
            .filter(expenses::narration.like(search_format))
            .get_results::<(Expense, Project)>(conn.deref_mut())
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        project_id: Uuid,
        amount: i64,
        narration: String,
        spent_at: chrono::NaiveDateTime,
    ) -> Expense {
        let model = Expense {
            expense_id: Uuid::new_v4(),
            user_id,
            project_id,
            amount: to_cent(amount),
            narration,
            spent_at,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(expenses::table)
            .values(model.clone())
            .execute(get_db_conn(pool).deref_mut())
            .expect(db_failed_to_execute());

        model
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
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_err() {
            return result;
        }

        diesel::update(expenses::dsl::expenses.filter(expenses::expense_id.eq(id)))
            .set((
                expenses::dsl::amount.eq(to_cent(amount)),
                expenses::dsl::narration.eq(narration),
                expenses::dsl::project_id.eq(project_id),
                expenses::dsl::spent_at.eq(spent_at),
            ))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to update expense");

        Ok(result.unwrap())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Expense> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_err() {
            return result;
        }

        diesel::update(expenses::dsl::expenses.filter(expenses::expense_id.eq(id)))
            .set(expenses::dsl::deleted_at.eq(current_timestamp()))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to delete expense");

        Ok(result.unwrap())
    }

    #[allow(dead_code)]
    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> QueryResult<Expense> {
        expenses::table
            .filter(expenses::expense_id.eq(id))
            .filter(expenses::deleted_at.is_null())
            .first::<Expense>(get_db_conn(pool).deref_mut())
    }

    pub fn find_owned_by_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
    ) -> QueryResult<Expense> {
        expenses::table
            .filter(expenses::expense_id.eq(id))
            .filter(expenses::user_id.eq(user_id))
            .filter(expenses::deleted_at.is_null())
            .first::<Expense>(get_db_conn(pool).deref_mut())
    }
}
