use std::ops::DerefMut;
use chrono::{Datelike, Utc};
use crate::helpers::db::current_timestamp;
use crate::helpers::error_messages::db_failed_to_execute;
use crate::helpers::{get_db_conn};
use crate::helpers::http::QueryParams;
use crate::models::DBPool;
use crate::schema::{budgets};
use diesel::{ExpressionMethods, OptionalExtension, PgTextExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use diesel::result::Error;
use uuid::Uuid;
use crate::helpers::date_time::Month;
use crate::helpers::db_pagination::Paginate;
use crate::models::budget::Budget;

pub struct BudgetRepository;

impl BudgetRepository {
    pub fn list_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        mut query_params: QueryParams,
    ) -> QueryResult<(Vec<Budget>, i64)> {
        let builder = budgets::table
            .filter(budgets::user_id.eq(id))
            .filter(budgets::deleted_at.is_null())
            .order_by(budgets::created_at.desc())
            .limit(query_params.get_limit());

        let search_format = format!("%{}%", query_params.get_search_query());

        builder
            .filter(budgets::title.ilike(search_format.clone()))
            .or_filter(budgets::comment.ilike(search_format.clone()))
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<Budget>(get_db_conn(pool).deref_mut())
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        amount: i64,
        month: i16,
        year: i16,
        comment: Option<String>,
    ) -> Budget {
        let model = Budget {
            user_id,
            amount,
            comment,
            month,
            year,
            amount_used: 0,
            title: make_budget_title(month, year),
            budget_id: Uuid::new_v4(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(budgets::table)
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
        amount: i64,
        month: i16,
        year: i16,
        comment: Option<String>,
    ) -> QueryResult<Budget> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_err() {
            return result;
        }

        diesel::update(budgets::dsl::budgets.filter(budgets::budget_id.eq(id)))
            .set((
                budgets::dsl::amount.eq(amount),
                budgets::dsl::comment.eq(comment),
                budgets::dsl::title.eq(make_budget_title(month, year)),
                budgets::dsl::month.eq(month),
                budgets::dsl::year.eq(year),
            ))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to update budget");

        Ok(result.unwrap())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Budget> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_err() {
            return result;
        }

        diesel::update(budgets::dsl::budgets.filter(budgets::budget_id.eq(id)))
            .set(budgets::dsl::deleted_at.eq(current_timestamp()))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to delete budget");

        Ok(result.unwrap())
    }

    #[allow(dead_code)]
    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> QueryResult<Budget> {
        budgets::table
            .filter(budgets::budget_id.eq(id))
            .filter(budgets::deleted_at.is_null())
            .first::<Budget>(get_db_conn(pool).deref_mut())
    }

    pub fn find_owned_by_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
    ) -> QueryResult<Budget> {
        budgets::table
            .filter(budgets::budget_id.eq(id))
            .filter(budgets::user_id.eq(user_id))
            .filter(budgets::deleted_at.is_null())
            .first::<Budget>(get_db_conn(pool).deref_mut())
    }

    pub fn find_owned_current_month_budget(&mut self, pool: &DBPool, user_id: Uuid) -> Result<Option<Budget>, Error> {
        let budget = budgets::table
            .filter(budgets::month.eq(Utc::now().month() as i16))
            .filter(budgets::user_id.eq(user_id))
            .filter(budgets::deleted_at.is_null())
            .first::<Budget>(get_db_conn(pool).deref_mut());

        return budget.optional();
    }
}

fn make_budget_title(month: i16, year: i16) -> String {
    format!("{}, {} Budget", Month::new(month.clone()).name().unwrap(), year)
}
