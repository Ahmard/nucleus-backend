use crate::helpers::auth::get_uuid;
use crate::helpers::http::{IdPathParam, QueryParams};
use crate::helpers::responder::{json_entity_not_found_response, json_error_message, json_invalid_uuid_response, json_pagination, json_success, json_success_message};
use crate::http::middlewares::auth_middleware::AuthMiddleware;
use crate::models::budget::BudgetForm;
use crate::models::DBPool;
use crate::repositories::budget_repository::BudgetRepository;
use crate::services::budget_service::BudgetService;
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpMessage, HttpRequest, HttpResponse};

pub fn budget_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
    cfg.service(show);
    cfg.service(update);
    cfg.service(delete);
}

#[get("")]
async fn index(
    pool: Data<DBPool>,
    req: HttpRequest,
    q: Query<QueryParams>,
    _: AuthMiddleware,
) -> HttpResponse {
    let user_id = get_uuid(req.extensions());
    let budgets = BudgetRepository.list_by_user_id(pool.get_ref(), user_id, q.into_inner());
    json_pagination(budgets.unwrap())
}

#[post("")]
async fn create(
    pool: Data<DBPool>,
    form: Json<BudgetForm>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let budget = BudgetService.create(
        pool.get_ref(),
        get_uuid(req.extensions()),
        form.amount,
        form.month,
        form.year,
        form.comment.clone(),
    );

    json_success(budget)
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

    let result = BudgetRepository.find_owned_by_id(
        pool.get_ref(),
        id.unwrap(),
        get_uuid(req.extensions()),
    );

    if result.is_err() {
        return json_entity_not_found_response("budget");
    }

    json_success(result.unwrap())
}

#[put("{id}")]
async fn update(
    pool: Data<DBPool>,
    form: Json<BudgetForm>,
    mut param: Path<IdPathParam>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    let result = BudgetService.update(
        pool.get_ref(),
        id.unwrap(),
        get_uuid(req.extensions()),
        form.amount,
        form.month,
        form.year,
        form.comment.clone(),
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

    BudgetService
        .delete(pool.get_ref(), id.unwrap(), get_uuid(req.extensions()))
        .expect("Failed to delete budget");

    json_success_message("budget deleted")
}
