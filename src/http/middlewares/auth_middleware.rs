use core::fmt;
use std::env;
use std::future::{ready, Ready};

use crate::models::user::User;
use crate::models::DBPool;
use crate::repositories::user_repository::UserRepository;
use actix_web::error::ErrorUnauthorized;
use actix_web::web::Data;
use actix_web::{dev::Payload, Error as ActixWebError};
use actix_web::{http, FromRequest, HttpMessage, HttpRequest};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::Serialize;

use crate::services::auth_service::TokenClaims;

#[derive(Debug, Serialize)]
struct ErrorResponse<'a> {
    success: bool,
    status: i32,
    message: &'a str,
}

impl fmt::Display for ErrorResponse<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

pub struct AuthMiddleware {
    pub user_id: uuid::Uuid,
}

impl FromRequest for AuthMiddleware {
    type Error = ActixWebError;
    type Future = Ready<Result<Self, Self::Error>>;
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        let token = req
            .cookie("token")
            .map(|c| c.value().to_string())
            .or_else(|| {
                req.headers()
                    .get(http::header::AUTHORIZATION)
                    .map(|h| h.to_str().unwrap().split_at(7).1.to_string())
            });

        if token.is_none() {
            return ready(Err(ErrorUnauthorized(make_unauthorized_response(
                "You are not logged in, please provide token",
            ))));
        }

        let claims = match decode::<TokenClaims>(
            &token.unwrap(),
            &DecodingKey::from_secret(env::var("APP_KEY").unwrap().as_ref()),
            &Validation::default(),
        ) {
            Ok(c) => c.claims,
            Err(_) => {
                return ready(Err(ErrorUnauthorized(make_unauthorized_response(
                    "Invalid auth token",
                ))));
            }
        };

        let user_id = uuid::Uuid::parse_str(claims.sub.as_str()).unwrap();
        let pool = req.app_data::<Data<DBPool>>().unwrap();
        let user_lookup = UserRepository.find_by_id(pool, user_id).unwrap();

        if user_lookup.is_none() {
            return ready(Err(ErrorUnauthorized(make_unauthorized_response(
                "Invalid auth token, user not found",
            ))));
        }

        req.extensions_mut()
            .insert::<uuid::Uuid>(user_id.to_owned());

        req.extensions_mut().insert::<User>(user_lookup.unwrap());

        ready(Ok(AuthMiddleware { user_id }))
    }
}

fn make_unauthorized_response(message: &str) -> ErrorResponse {
    ErrorResponse {
        success: false,
        status: 401,
        message,
    }
}
