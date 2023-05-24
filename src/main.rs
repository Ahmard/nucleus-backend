use std::env;

use actix_files::Files;
use actix_session::config::PersistentSession;
use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{cookie, web, App, HttpResponse, HttpServer};
use cookie::time::Duration;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use env_logger::Env;
use tera::Tera;

use crate::http::kernel::{register_middlewares, register_routes, setup_cors};
use crate::models::DBPool;

mod helpers;
mod http;
mod macros;
mod models;
mod repositories;
mod schema;
mod services;

#[derive(Debug, Clone)]
pub struct AppState {
    tera: Tera,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host: String = env::var("HOST").unwrap();
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();
    let db_url: String = env::var("DATABASE_URL").unwrap();

    // create db connection pool
    let manager = ConnectionManager::<MysqlConnection>::new(db_url);
    let pool: DBPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let domain: String = env::var("DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();
    let app_state = AppState { tera };

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Server started at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(app_state.clone()))
            .service(Files::new("/static", "./static"))
            .configure(register_routes)
            .configure(register_middlewares)
            // .wrap(middleware::NormalizePath::new(TrailingSlash::MergeOnly))
            .wrap(Logger::default())
            .wrap(setup_cors())
            .default_service(web::to(|| async {
                HttpResponse::Ok()
                    .status(StatusCode::NOT_FOUND)
                    .body("Page Not Found")
            }))
            .wrap(
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(false)
                    // customize session and cookie expiration
                    .session_lifecycle(PersistentSession::default().session_ttl(Duration::days(30)))
                    .cookie_name("nucleus-auth".to_owned())
                    .cookie_domain(Some(domain.clone()))
                    .cookie_path("/".to_owned())
                    .build(),
            )
    })
    .shutdown_timeout(1)
    .bind((host, port))?
    .run()
    .await
}
