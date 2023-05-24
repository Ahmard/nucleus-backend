use crate::http::controllers::auth_controller::auth_controller;
use crate::http::controllers::main_controller::main_controller;
use crate::http::controllers::project_controller::project_controller;
use actix_web::web::ServiceConfig;

mod auth_controller;
mod main_controller;
mod project_controller;

pub struct Controller {
    pub path: String,
    pub handler: fn(cfg: &mut ServiceConfig),
}

pub struct Route {
    pub prefix: String,
    pub controllers: Vec<Controller>,
}

pub fn routes() -> Vec<Route> {
    let mut routes = Vec::new();

    routes.push(Route {
        prefix: String::from("/test"),
        controllers: vec![Controller {
            path: String::from(""),
            handler: main_controller,
        }],
    });

    routes.push(Route {
        prefix: String::from("/api/v1"),
        controllers: vec![
            Controller {
                path: String::from("/auth"),
                handler: auth_controller,
            },
            Controller {
                path: String::from("/projects"),
                handler: project_controller,
            },
        ],
    });

    return routes;
}
