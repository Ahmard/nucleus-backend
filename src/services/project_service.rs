use uuid::Uuid;

use crate::core::enums::http_error::DBResult;
use crate::models::project::{Project, ProjectForm};
use crate::models::DBPool;
use crate::repositories::project_repository::ProjectRepository;

pub struct ProjectService;

impl ProjectService {
    pub fn create(&mut self, pool: &DBPool, user_id: Uuid, form: ProjectForm) -> Project {
        ProjectRepository.create(pool, user_id, form)
    }

    pub fn update(
        &mut self,
        pool: &DBPool,
        id: Uuid,
        user_id: Uuid,
        form: ProjectForm,
    ) -> DBResult<Project> {
        ProjectRepository.update(pool, id, user_id, form)
    }

    pub fn delete(&mut self, pool: &DBPool, id: Uuid, user_id: Uuid) -> DBResult<Project> {
        ProjectRepository.delete(pool, id, user_id)
    }
}
