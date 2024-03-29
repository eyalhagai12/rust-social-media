use actix_web::web;

use crate::resources::users::register;

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/users").route("/register", web::post().to(register)));
}
