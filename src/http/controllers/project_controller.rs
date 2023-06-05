use crate::core::enums::http_error::ErroneousOption;
use crate::core::helpers::auth::get_auth_id;
use crate::core::helpers::http::{IdPathParam, QueryParams};
use crate::core::helpers::responder::{
    json_entity_not_found_response, json_error_message, json_invalid_uuid_response,
    json_pagination, json_success, json_success_message,
};
use crate::http::middlewares::auth_middleware::AuthMiddleware;
use crate::models::project::ProjectForm;
use crate::models::DBPool;
use crate::repositories::expense_repository::ExpenseRepository;
use crate::repositories::project_repository::ProjectRepository;
use crate::services::project_service::ProjectService;
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpMessage, HttpRequest, HttpResponse};

pub fn project_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(show);
    cfg.service(aggregate);
    cfg.service(update);
    cfg.service(delete);
    cfg.service(expenses);
}

#[get("")]
async fn index(
    pool: Data<DBPool>,
    req: HttpRequest,
    q: Query<QueryParams>,
    _: AuthMiddleware,
) -> HttpResponse {
    let user_id = get_auth_id(req.extensions());
    let projects = ProjectRepository.list_by_user_id(pool.get_ref(), user_id, q.into_inner());
    json_pagination(projects.unwrap())
}

#[post("")]
async fn create(
    pool: Data<DBPool>,
    form: Json<ProjectForm>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let project = ProjectService.create(
        pool.get_ref(),
        get_auth_id(req.extensions()),
        form.into_inner(),
    );

    json_success(project)
}

#[get("{id}")]
async fn show(
    pool: Data<DBPool>,
    mut param: Path<IdPathParam>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    let result = ProjectRepository.find_owned_by_id(
        pool.get_ref(),
        id.unwrap(),
        get_auth_id(req.extensions()),
    );

    if result.is_err() {
        return json_entity_not_found_response("project");
    }

    json_success(result.unwrap_entity())
}

#[get("{id}/aggregates")]
async fn aggregate(
    pool: Data<DBPool>,
    mut param: Path<IdPathParam>,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    let result = ProjectRepository.fetch_aggregate_by_project_id(pool.get_ref(), id.unwrap());

    if result.is_err() {
        return json_error_message(result.err().unwrap().to_string().as_str());
    }

    json_success(result.unwrap().first().unwrap())
}

#[put("{id}")]
async fn update(
    pool: Data<DBPool>,
    form: Json<ProjectForm>,
    mut param: Path<IdPathParam>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    let result = ProjectService.update(
        pool.get_ref(),
        id.unwrap(),
        get_auth_id(req.extensions()),
        form.into_inner(),
    );

    if result.is_err() {
        return json_error_message(result.err().unwrap().to_string().as_str());
    }

    json_success(result.unwrap())
}

#[delete("{id}")]
async fn delete(
    pool: Data<DBPool>,
    mut param: Path<IdPathParam>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    ProjectService
        .delete(pool.get_ref(), id.unwrap(), get_auth_id(req.extensions()))
        .expect("Failed to delete project");

    json_success_message("project deleted")
}

#[get("{id}/expenses")]
async fn expenses(
    pool: Data<DBPool>,
    mut param: Path<IdPathParam>,
    q: Query<QueryParams>,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    let projects =
        ExpenseRepository.list_by_project_id(pool.get_ref(), id.unwrap(), q.into_inner());
    json_pagination(projects.unwrap())
}
