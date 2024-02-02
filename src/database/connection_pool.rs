use std::env;

use diesel::{r2d2::{self, ConnectionManager, Pool}, PgConnection};

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

pub fn init_connection_pool() -> DbPool {
    dotenvy::dotenv().ok();

    let connection_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = r2d2::ConnectionManager::<PgConnection>::new(connection_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to connect to postgres database")
}