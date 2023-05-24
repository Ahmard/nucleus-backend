use crate::http::controllers::routes;
use actix_cors::Cors;
use actix_web::http::header;
use actix_web::web;
use actix_web::web::ServiceConfig;
use std::env;

struct Route {
    route: String,
    controller: fn(cfg: &mut ServiceConfig),
}

pub fn register_routes(actix_config: &mut ServiceConfig) {
    log::debug!("Discovering routes...");
    let mut registered = vec![];

    for route in routes() {
        for controller in route.controllers {
            registered.push(Route {
                route: route.prefix.as_str().to_owned() + controller.path.as_str(),
                controller: controller.handler,
            });
        }
    }

    for route in registered {
        let path = route.route.as_str();
        log::debug!("Route Group: {}", path);

        if path.len() == 0 {
            actix_config.configure(route.controller);
        } else {
            actix_config.service(web::scope(path).configure(route.controller));
        }
    }

    log::debug!("Route discovery finished :)");
}

pub fn setup_cors() -> Cors {
    Cors::default()
        .allowed_origin(env::var("FRONTEND_ADDRESS").unwrap().as_str())
        .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
        .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
        .allowed_header(header::CONTENT_TYPE)
        .max_age(3600)
}

pub fn register_middlewares(_actix_config: &mut ServiceConfig) {
    // for middleware in middlewares() {
    // }
}
