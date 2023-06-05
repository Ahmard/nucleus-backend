use argon2::Config;
use std::env;

pub fn password_hash(password: String) -> String {
    let salt = env::var("APP_KEY").unwrap();
    let config = Config::default();

    argon2::hash_encoded(password.as_bytes(), salt.as_bytes(), &config).unwrap()
}

pub fn password_verify(hash: &str, password: &str) -> bool {
    argon2::verify_encoded(hash, password.as_bytes()).unwrap()
}
