use actix_web::{web::{self, ReqData}, HttpResponse, Responder};
use crate::{database::{connection_pool::DbPool, models::{posts::get_all_user_posts, users::User}}, logging::Logger};

pub async fn get_user_posts(pool: web::Data<DbPool>, attached_user: ReqData<User>) -> impl Responder {
    let user = attached_user.into_inner();
    let logger = Logger::new("GetUserPosts");
    logger.debug(format!("getting all posts for {}", user.display_name).as_str()).await.expect("failed to log");
    let mut conn = pool.get().expect("Failed to get database connection");

    get_all_user_posts(&mut conn, user.id);

    HttpResponse::Ok()
}