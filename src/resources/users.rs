use actix_web::{web, HttpResponse, Responder};
use serde_json::json;

use crate::{
    auth::jwt::encode_user, database::{connection_pool::DbPool, models::users}, logging, responses::user::UserResponse, schemas::{login_schema::UserLoginSchema, new_user::NewUserSchema}
};

pub async fn register(pool: web::Data<DbPool>, schema: web::Json<NewUserSchema>) -> impl Responder {
    let logger = logging::Logger::new("Register");
    let new_user_schema = schema.into_inner();
    let mut conn = pool.get().expect("Failed to get database connection");

    let user_result = users::create_new_user(&mut conn, new_user_schema);

    match user_result {
        Ok(user) => {
            logger
                .debug(format!("{} has been successfully registered!", user.display_name).as_str())
                .await
                .expect("failed to log user registration");
            let token = encode_user(&user);
            HttpResponse::Ok().json(json!({ "token": token }))
        }
        Err(err) => {
            logger
                .error(err.to_string().as_str())
                .await
                .expect("failed to log user registration failure");
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login(pool: web::Data<DbPool>, schema: web::Json<UserLoginSchema>) -> impl Responder {
    let logger = logging::Logger::new("Login");
    let user_login_schema = schema.into_inner();
    let mut conn = pool.get().expect("Failed to get database connection");

    let user_result = users::get_user_with_credentials(&mut conn, user_login_schema);

    match user_result {
        Ok(user) => {
            logger
                .debug(format!("{} has been successfully logged in!", user.display_name).as_str())
                .await
                .expect("failed to log user log in");
            let user_response = UserResponse {
                display_name: user.display_name,
                username: user.username,
                email: user.email,
            }; // this can be deleted if i use `select` in the query instead
            HttpResponse::Ok().json(user_response)
        }
        Err(err) => {
            logger
                .error(err.to_string().as_str())
                .await
                .expect("failed to log user log in failure");
            HttpResponse::InternalServerError().finish()
        }
    }
}
