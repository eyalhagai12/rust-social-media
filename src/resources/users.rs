use actix_web::{web, HttpResponse, Responder};
use log::debug;

use crate::{database::{connection_pool::DbPool, models::users}, schemas::new_user::NewUserSchema};

pub async fn register(pool: web::Data<DbPool>, schema: web::Json<NewUserSchema>) -> impl Responder {
    let new_user_schema = schema.into_inner();
    let mut conn = pool.get().expect("Failed to get database connection");
    
    let user_result = users::create_new_user(&mut conn, new_user_schema);

    match user_result {
        Ok(user) => {
            debug!("{} was successfully registered", user.display_name);
            HttpResponse::Ok().json(user)
        },
        Err(_) => {
            debug!("registration failed");
            HttpResponse::InternalServerError().finish()
        }
    }
}
