use crate::AppState;
use actix_web::body::BoxBody;
use actix_web::http::header::ContentType;
use actix_web::http::StatusCode;
use actix_web::web::Data;
use actix_web::{HttpRequest, HttpResponse, Responder as ActixResponder};
use serde::{Deserialize, Serialize};
use tera::Context;

#[derive(Serialize, Deserialize)]
pub struct JsonResponse<T: Serialize> {
    success: bool,
    data: T,
    status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct JsonPaginationResponse<T: Serialize> {
    success: bool,
    data: T,
    pages: i64,
    status: u16,
}

#[derive(Serialize, Deserialize)]
pub struct JsonSuccessMessageResponse {
    message: String,
}

pub fn json<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    HttpResponse::build(status).json(data)
}

pub fn json_success<T: Serialize>(data: T) -> HttpResponse {
    json(
        JsonResponse {
            success: true,
            status: 200,
            data,
        },
        StatusCode::OK,
    )
}

pub fn json_pagination<T: Serialize>(data: (T, i64)) -> HttpResponse {

    json(
        JsonPaginationResponse {
            success: true,
            status: 200,
            data: data.0,
            pages: data.1,
        },
        StatusCode::OK,
    )
}

pub fn json_error<T: Serialize>(data: T, status: StatusCode) -> HttpResponse {
    json(
        JsonResponse {
            success: false,
            status: status.as_u16(),
            data,
        },
        status,
    )
}

pub fn json_error_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::BAD_REQUEST)
}

pub fn json_error_message_status(message: &str, status: StatusCode) -> HttpResponse {
    json_error(
        JsonSuccessMessageResponse {
            message: message.to_string(),
        },
        status,
    )
}

pub fn json_success_message(message: &str) -> HttpResponse {
    json_success(JsonSuccessMessageResponse {
        message: message.to_string(),
    })
}

pub fn json_unauthorized_message(message: &str) -> HttpResponse {
    json_error_message_status(message, StatusCode::UNAUTHORIZED)
}

pub fn json_invalid_uuid_response() -> HttpResponse {
    json_error_message("Your provided ID is invalid, please inspect it")
}

pub fn json_entity_not_found_response(entity: &str) -> HttpResponse {
    json_error_message(format!("Such {} does not exists", entity).as_str())
}

pub enum Responder {
    HtmlTemplate(String, Context, StatusCode),
    Redirect(String, StatusCode),
}

impl ActixResponder for Responder {
    type Body = BoxBody;

    fn respond_to(self, req: &HttpRequest) -> HttpResponse<Self::Body> {
        match self {
            Responder::Redirect(location, status) => HttpResponse::build(status)
                .append_header(("Location", location))
                .finish(),
            Responder::HtmlTemplate(file, data, status) => {
                let mut filename = file.to_owned();
                if !filename.ends_with(".tera.html") {
                    filename.push_str(".tera.html");
                }

                let app_state = req.app_data::<Data<AppState>>().unwrap();
                let html = match app_state.tera.render(&filename, &data) {
                    Ok(string) => string,
                    Err(error) => {
                        eprintln!("Failed to render template: {}", error);
                        ::std::process::exit(1);
                    }
                };

                HttpResponse::build(status)
                    .content_type(ContentType::html())
                    .body(html)
            }
        }
    }
}
