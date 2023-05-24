use crate::helpers::responder::{json_success_message, Responder};
use crate::macros::http_response::{redirect, view};
use actix_web::web::ServiceConfig;
use actix_web::{get, HttpRequest, HttpResponse};

pub fn main_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(about);
    cfg.service(redirect);
}

#[get("")]
async fn index(_req: HttpRequest) -> Responder {
    view!("index", {
        "name": "Ahmad",
        "level": 3
    })
}

#[get("about")]
async fn about() -> HttpResponse {
    json_success_message("About Page")
}

#[get("redirect")]
async fn redirect() -> Responder {
    redirect!("/")
}
