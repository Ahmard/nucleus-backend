use crate::core::helpers::responder::json_success_message;
use actix_web::web::ServiceConfig;
use actix_web::{get, HttpRequest, HttpResponse};

pub fn main_controller(cfg: &mut ServiceConfig) {
    cfg.service(index);
    cfg.service(about);
}

#[get("")]
async fn index(_req: HttpRequest) -> HttpResponse {
    json_success_message("Hello")
}

#[get("about")]
async fn about() -> HttpResponse {
    json_success_message("About Page")
}
