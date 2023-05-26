use std::ops::DerefMut;
use crate::helpers::db::current_timestamp;
use crate::helpers::error_messages::db_failed_to_execute;
use crate::helpers::{get_db_conn};
use crate::helpers::http::QueryParams;
use crate::models::project::Project;
use crate::models::DBPool;
use crate::schema::projects;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};
use uuid::Uuid;
use crate::helpers::db_pagination::Paginate;

pub struct ProjectRepository;

impl ProjectRepository {
    pub fn list_by_user_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        mut query_params: QueryParams,
    ) -> QueryResult<(Vec<Project>, i64)> {
        let builder = projects::table
            .filter(projects::user_id.eq(id))
            .filter(projects::deleted_at.is_null())
            .order_by(projects::created_at.desc())
            .limit(query_params.get_limit());

        let search_format = format!("%{}%", query_params.get_search_query());
        builder
            .filter(projects::name.like(search_format))
            .paginate(query_params.get_page())
            .per_page(query_params.get_per_page())
            .load_and_count_pages::<Project>(get_db_conn(pool).deref_mut())
    }

    pub fn create(
        &mut self,
        pool: &DBPool,
        user_id: Uuid,
        name: String,
        description: String,
    ) -> Project {
        let model = Project {
            project_id: Uuid::new_v4(),
            user_id,
            name,
            description,
            created_at: current_timestamp(),
            updated_at: current_timestamp(),
            deleted_at: None,
        };

        diesel::insert_into(projects::table)
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
        name: String,
        description: String,
    ) -> QueryResult<Project> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_err() {
            return result;
        }

        diesel::update(projects::dsl::projects.filter(projects::project_id.eq(id)))
            .set((
                projects::dsl::name.eq(name),
                projects::dsl::description.eq(description),
            ))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to update project");

        Ok(result.unwrap())
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Project> {
        let result = self.find_owned_by_id(pool, id, user_id);

        if result.is_err() {
            return result;
        }

        diesel::update(projects::dsl::projects.filter(projects::project_id.eq(id)))
            .set(projects::dsl::deleted_at.eq(current_timestamp()))
            .execute(get_db_conn(pool).deref_mut())
            .expect("Failed to delete project");

        Ok(result.unwrap())
    }

    #[allow(dead_code)]
    pub fn find_by_id(&mut self, pool: &DBPool, id: Uuid) -> QueryResult<Project> {
        projects::table
            .filter(projects::project_id.eq(id))
            .filter(projects::deleted_at.is_null())
            .first::<Project>(get_db_conn(pool).deref_mut())
    }

    pub fn find_owned_by_id(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
    ) -> QueryResult<Project> {
        projects::table
            .filter(projects::project_id.eq(id))
            .filter(projects::user_id.eq(user_id))
            .filter(projects::deleted_at.is_null())
            .first::<Project>(get_db_conn(pool).deref_mut())
    }
}
