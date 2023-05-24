use crate::models::project::Project;
use crate::models::DBPool;
use crate::repositories::project_repository::ProjectRepository;
use diesel::QueryResult;
use uuid::Uuid;

pub struct ProjectService;

impl ProjectService {
    pub fn create(&mut self, pool: &DBPool, user_id: Uuid, name: String, desc: String) -> Project {
        ProjectRepository.create(pool, user_id, name, desc)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        name: String,
        desc: String,
    ) -> QueryResult<Project> {
        ProjectRepository.update(pool, id, user_id, name, desc)
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> QueryResult<Project> {
        ProjectRepository.delete(pool, id, user_id)
    }
}
