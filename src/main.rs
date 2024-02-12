use actix_web::{web, App, HttpServer};
use env_logger::Env;
use resources::healthcheck::healthcheck;
use routes::config_routes;

mod auth;
mod database;
mod logging;
mod resources;
mod responses;
mod routes;
mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("debug"));
    let pool = database::connection_pool::init_connection_pool();
    let app_data = web::Data::new(pool);

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .configure(config_routes)
            .route("/healthcheck", web::get().to(healthcheck))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
