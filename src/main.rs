use actix_web::{middleware::Logger, web, App, HttpServer};
use env_logger::Env;
use resources::healthcheck::healthcheck;

mod database;
mod resources;
mod responses;
mod schemas;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .route("/healthcheck", web::get().to(healthcheck))
    })
    .bind(("127.0.0.1", 5000))?
    .run()
    .await
}
