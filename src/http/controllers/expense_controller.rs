use crate::helpers::auth::get_user_id;
use crate::helpers::db::current_timestamp;
use crate::helpers::http::{IdPathParam, QueryParams};
use crate::helpers::responder::{
    json_error_message, json_invalid_uuid_response, json_success, json_success_message,
};
use crate::http::middlewares::auth_middleware::AuthMiddleware;
use crate::models::expense::ExpenseForm;
use crate::models::DBPool;
use crate::repositories::expense_repository::ExpenseRepository;
use crate::services::expense_service::ExpenseService;
use actix_web::web::{Data, Json, Path, Query, ServiceConfig};
use actix_web::{delete, get, post, put, HttpMessage, HttpRequest, HttpResponse};
use chrono::NaiveDateTime;
use log::info;
use std::str::FromStr;
use uuid::Uuid;

pub fn expense_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(create);
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
    let user_id = get_user_id(req.extensions());
    let expenses = ExpenseRepository.list_by_user_id(pool.get_ref(), user_id, q.into_inner());
    json_success(expenses.unwrap())
}

#[post("")]
async fn create(
    pool: Data<DBPool>,
    form: Json<ExpenseForm>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let expense = ExpenseService.create(
        pool.get_ref(),
        get_user_id(req.extensions()),
        Uuid::from_str(form.project_id.as_str()).unwrap(),
        form.amount.clone(),
        form.narration.clone(),
        get_spending_time(form.spent_at.clone()),
    );

    json_success(expense)
}

#[put("{id}")]
async fn update(
    pool: Data<DBPool>,
    form: Json<ExpenseForm>,
    mut param: Path<IdPathParam>,
    req: HttpRequest,
    _: AuthMiddleware,
) -> HttpResponse {
    let id = param.get_uuid();
    if id.is_err() {
        return json_invalid_uuid_response();
    }

    let result = ExpenseService.update(
        pool.get_ref(),
        id.unwrap(),
        get_user_id(req.extensions()),
        Uuid::from_str(form.project_id.as_str()).unwrap(),
        form.amount.clone(),
        form.narration.clone(),
        get_spending_time(form.spent_at.clone()),
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

    ExpenseService
        .delete(pool.get_ref(), id.unwrap(), get_user_id(req.extensions()))
        .expect("Failed to delete expense");

    json_success_message("expense deleted")
}

fn get_spending_time(spent_at: Option<String>) -> NaiveDateTime {
    let spending_time = spent_at.clone();
    match spending_time {
        None => current_timestamp(),
        Some(val) => NaiveDateTime::parse_from_str(val.as_str(), "%Y-%m-%d %H:%M:%S").unwrap(),
    }
}