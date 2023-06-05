use crate::core::helpers::string::password_verify;
use crate::models::user::UserStatus;
use crate::models::DBPool;
use crate::repositories::user_repository::{user_status_is, UserRepository};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

pub struct AuthService;

#[derive(Serialize)]
pub struct AuthAccessData {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub iat: usize,
    pub exp: usize,
}

impl AuthService {
    pub fn login(
        &mut self,
        pool: &DBPool,
        email: String,
        password: String,
    ) -> Result<AuthAccessData, String> {
        let user_lookup = UserRepository.find_by_email(pool, email);
        let context_less_error_message = Err(String::from("Invalid email address or password"));

        if user_lookup.is_err() {
            return context_less_error_message;
        }

        let user = user_lookup.unwrap();

        if !password_verify(user.password.as_str(), password.as_str()) {
            return context_less_error_message;
        }

        if user_status_is(user.status.to_owned(), UserStatus::Pending) {
            return Err(String::from("Your account is not activated yet"));
        }

        if user_status_is(user.status, UserStatus::Inactive) {
            return Err(String::from("Your account is not active"));
        }

        let token_lifetime_in_minutes: i64 =
            env::var("AUTH_TOKEN_LIFETIME").unwrap().parse().unwrap();

        let now = Utc::now();
        let iat = now.timestamp() as usize;
        let exp = (now + Duration::minutes(token_lifetime_in_minutes)).timestamp() as usize;
        let claims: TokenClaims = TokenClaims {
            sub: user.user_id.clone().to_string(),
            exp,
            iat,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(env::var("APP_KEY").unwrap().as_ref()),
        )
        .unwrap();

        Ok(AuthAccessData {
            access_token: token,
            token_type: "bearer".to_string(),
            expires_in: token_lifetime_in_minutes.to_owned(),
        })
    }

    pub fn logout(&mut self) {}
}
