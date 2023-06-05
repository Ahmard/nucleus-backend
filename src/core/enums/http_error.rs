use std::fmt::{Debug, Display, Formatter};

use actix_web::{
    http::{header::ContentType, StatusCode},
    HttpResponse, ResponseError,
};
use diesel::result::Error;
use diesel::QueryResult;

use crate::core::enums::http_error::HttpStatus::EntityNotFound;
use crate::core::helpers::responder::{json_entity_not_found_response, json_error_message_status};

pub enum HttpStatus {
    DBOperationFailed,
    EntityNotFound(String),
}

pub fn format_message(status: &HttpStatus, f: &mut Formatter<'_>) -> std::fmt::Result {
    match status {
        HttpStatus::DBOperationFailed => f.write_str("Database operation failed"),
        EntityNotFound(entity) => f.write_str(format!("Such {} does not exits", entity).as_str()),
    }
}

pub fn send_response(status: &HttpStatus) -> HttpResponse {
    match status {
        EntityNotFound(entity) => json_entity_not_found_response(entity),
        _ => HttpResponse::build(status.status_code())
            .insert_header(ContentType::html())
            .body(status.to_string()),
    }
}

pub trait OptionalResult<'a, T> {
    fn optional(self, entity: &'a str) -> Result<Option<T>, HttpStatus>;
}

pub trait ErroneousOption<T> {
    fn is_error_or_empty(&self) -> bool;

    fn get_error_result(self) -> DBResult<T>;

    fn send_error(self) -> HttpResponse;

    fn unwrap_entity(self) -> T;
}

impl<'a, T> OptionalResult<'a, T> for QueryResult<T> {
    fn optional(self, entity: &'a str) -> DBResult<Option<T>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(Error::NotFound) => Err(EntityNotFound(entity.to_string())),
            Err(_e) => Err(HttpStatus::DBOperationFailed),
        }
    }
}

impl<T> ErroneousOption<T> for DBResult<Option<T>> {
    fn is_error_or_empty(&self) -> bool {
        self.as_ref().is_err() || self.as_ref().unwrap().is_none()
    }

    fn get_error_result(self) -> DBResult<T> {
        if self.is_err() {
            return Err(self.err().unwrap());
        }

        // let entity = self.
        panic!("Cannot acquire error on successful database action")
    }

    fn send_error(self) -> HttpResponse {
        if self.is_err() {
            return send_response(&self.err().unwrap());
        }

        json_error_message_status("Internal Server Error", StatusCode::INTERNAL_SERVER_ERROR)
    }

    fn unwrap_entity(self) -> T {
        if self.is_error_or_empty() {
            panic!("Cannot unwrap entity while current operation is either erroneous or result is empty")
        }

        self.unwrap().unwrap()
    }
}

pub type DBResult<T> = Result<T, HttpStatus>;

impl Debug for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_message(self, f)
    }
}

impl Display for HttpStatus {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        format_message(self, f)
    }
}

impl ResponseError for HttpStatus {
    fn status_code(&self) -> StatusCode {
        match self {
            EntityNotFound(_msg) => StatusCode::NOT_FOUND,
            HttpStatus::DBOperationFailed => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        send_response(self)
    }
}
