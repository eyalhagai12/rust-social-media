use actix_web::{web::{self, ReqData}, HttpResponse, Responder};
use serde_json::json;
use crate::{database::{connection_pool::DbPool, models::{posts::get_all_user_posts, users::User}}, logging::Logger};

pub async fn get_user_posts(pool: web::Data<DbPool>, attached_user: ReqData<User>) -> impl Responder {
    let user = attached_user.into_inner();
    let logger = Logger::new("GetUserPosts");
    let mut conn = pool.get().expect("Failed to get database connection");

    let posts_result = get_all_user_posts(&mut conn, user.id);

    match posts_result {
        Ok(posts) => {
            logger.debug(format!("got all posts for {}", user.display_name).as_str()).await.expect("failed to log");
            HttpResponse::Ok().json(posts)
        }
        Err(err) => {
            logger.error(err.to_string().as_str()).await.expect("failed to log");
            HttpResponse::InternalServerError().json(json!({"error": err.to_string()}))
        }
    }
}