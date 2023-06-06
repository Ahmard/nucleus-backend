use std::env;

use actix_files::Files;
use actix_web::http::StatusCode;
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{web, App, HttpResponse, HttpServer};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use env_logger::Env;
use tera::Tera;

use crate::http::kernel::{register_middlewares, register_routes, setup_cors};
use crate::models::DBPool;

mod core;
mod http;
mod models;
mod repositories;
mod schema;
mod services;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();

    let host: String = env::var("HOST").unwrap();
    let port: u16 = env::var("PORT").unwrap().parse().unwrap();

    let db_url: String = format!(
        "{}://{}:{}@{}:{}/{}",
        env::var("DB_DRIVER").unwrap(),
        env::var("DB_USERNAME").unwrap(),
        env::var("DB_PASSWORD").unwrap(),
        env::var("DB_HOST").unwrap(),
        env::var("DB_PORT").unwrap(),
        env::var("DB_DATABASE").unwrap(),
    );

    println!("DB URL: {}", db_url.clone());

    // create db connection pool
    let manager = ConnectionManager::<PgConnection>::new(db_url);
    let pool: DBPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");

    let tera = Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/templates/**/*")).unwrap();

    env::set_var("RUST_LOG", "debug");
    env::set_var("RUST_BACKTRACE", "1");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    log::info!("Server started at http://localhost:{}", port);

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(pool.clone()))
            .app_data(Data::new(tera.clone()))
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
    })
    .shutdown_timeout(1)
    .bind((host, port))?
    .run()
    .await
}
