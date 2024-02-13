use actix_web::web;

use crate::resources::{posts::get_user_posts, users::{login, register}};

pub fn config_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/users")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );

    cfg.service(
        web::resource("posts")
        .get(get_user_posts)
    );
}
