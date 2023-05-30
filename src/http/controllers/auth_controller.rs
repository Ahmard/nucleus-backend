use actix_web::http::StatusCode;
use actix_web::web::{Data, Json, ServiceConfig};
use actix_web::{get, post, HttpMessage, HttpRequest, HttpResponse};
use diesel::result::DatabaseErrorInformation;
use crate::helpers::auth::{get_auth_user};

use crate::helpers::responder::{json, json_error_message, json_success, json_success_message, json_unauthorized_message};
use crate::http::middlewares::auth_middleware::AuthMiddleware;
use crate::models::DBPool;

use crate::models::user::{LoginForm, RegisterForm};
use crate::repositories::user_repository::UserRepository;
use crate::services::auth_service::AuthService;

pub fn auth_controller(cfg: &mut ServiceConfig) {
    cfg.service(login);
    cfg.service(me);
    cfg.service(logout);
    cfg.service(register);
}

#[post("login")]
async fn login(pool: Data<DBPool>, data: Json<LoginForm>) -> HttpResponse {
    let result = AuthService.login(pool.get_ref(), data.email.clone(), data.password.clone());

    if result.is_err() {
        return json_unauthorized_message(result.err().unwrap().as_str());
    }

    json(result.unwrap(), StatusCode::OK)
}

#[get("me")]
async fn me(req: HttpRequest, _: AuthMiddleware) -> HttpResponse {
    let user = get_auth_user(req.extensions());
    json(user, StatusCode::OK)
}

#[post("logout")]
async fn logout() -> HttpResponse {
    AuthService.logout();
    json_success_message("Logged out successfully")
}

#[post("register")]
async fn register(pool: Data<DBPool>, form: Json<RegisterForm>) -> HttpResponse {
    let result =
        actix_web::web::block(move || UserRepository.create(pool.get_ref(), form.into_inner()))
            .await
            .expect("Failed to create user");

    if result.is_err() {
        return json_error_message(result.err().unwrap().message());
    }

    let mut user = result.unwrap();

    user.password = String::from("");
    json_success(user)
}
