use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use logging::logger::LOGGER;
use resources::healthcheck::healthcheck;
use routes::config_routes;
use slog::info;

mod database;
mod resources;
mod responses;
mod routes;
mod schemas;
mod logging;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    info!(LOGGER.logger, "This is the log from slog");

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
