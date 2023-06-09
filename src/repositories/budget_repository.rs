use crate::core::enums::http_error::{DBResult, ErroneousOption, OptionalResult};
use crate::core::helpers::date_time::Month;
use crate::core::helpers::db::current_timestamp;
use crate::core::helpers::db_pagination::{Paginate, PaginationResult};
use crate::core::helpers::get_db_conn;
use crate::core::helpers::http::QueryParams;
use crate::models::budget::{Budget, BudgetForm};
use crate::models::DBPool;
use crate::schema::budgets;
use chrono::{Datelike, Utc};
use diesel::{ExpressionMethods, PgTextExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use std::ops::DerefMut;
use uuid::Uuid;

pub struct BudgetRepository;

impl BudgetRepository {
    pub fn list_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        mut query_params: QueryParams,
    ) -> QueryResult<PaginationResult<Budget>> {
        let builder = budgets::table
            .filter(budgets::user_id.eq(id))
            .filter(budgets::deleted_at.is_null())
            .order_by(budgets::created_at.desc());

        let search_format = format!("%{}%", query_params.get_search_query());

        builder
            .filter(budgets::title.ilike(search_format.clone()))
            .or_filter(budgets::comment.ilike(search_format))
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<Budget>(get_db_conn(pool).deref_mut())
    }

    pub fn create(&mut self, pool: &DBPool, user_id: Uuid, form: BudgetForm) -> Budget {
        let model = Budget {
            user_id,
            amount: form.amount,
            comment: form.comment,
            month: form.month,
            year: form.year,
            amount_used: 0,
            title: make_budget_title(form.month, form.year),
            budget_id: Uuid::new_v4(),
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(budgets::table)
            .values(model.clone())
            .execute(get_db_conn(pool).deref_mut())
            .unwrap();

        model
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        form: BudgetForm,
    ) -> DBResult<Budget> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        diesel::update(budgets::dsl::budgets.filter(budgets::budget_id.eq(id)))
            .set((
                budgets::dsl::amount.eq(form.amount),
                budgets::dsl::comment.eq(form.comment),
                budgets::dsl::title.eq(make_budget_title(form.month, form.year)),
                budgets::dsl::month.eq(form.month),
                budgets::dsl::year.eq(form.year),
            ))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to update budget");

        Ok(result.unwrap_entity())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> DBResult<Budget> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_error_or_empty() {
            return result.get_error_result();
        }

        diesel::update(budgets::dsl::budgets.filter(budgets::budget_id.eq(id)))
            .set(budgets::dsl::deleted_at.eq(current_timestamp()))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to delete budget");

        Ok(result.unwrap_entity())
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
    ) -> DBResult<Option<Budget>> {
        budgets::table
            .filter(budgets::budget_id.eq(id))
            .filter(budgets::user_id.eq(user_id))
            .filter(budgets::deleted_at.is_null())
            .first::<Budget>(get_db_conn(pool).deref_mut())
            .optional("budget")
    }

    pub fn find_owned_current_month_budget(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
    ) -> DBResult<Option<Budget>> {
        let budget = budgets::table
            .filter(budgets::month.eq(Utc::now().month() as i16))
            .filter(budgets::user_id.eq(user_id))
            .filter(budgets::deleted_at.is_null())
            .first::<Budget>(get_db_conn(pool).deref_mut());

        budget.optional("budget")
    }
}

fn make_budget_title(month: i16, year: i16) -> String {
    format!("{}, {} Budget", Month::new(month).name().unwrap(), year)
}
