// use diesel::{
//     query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, QueryResult, RunQueryDsl
// };use crate::database::{models::posts::Post, schema::posts};

// pub fn get_user_posts(conn: &mut PgConnection, user_id: i32) -> QueryResult<Post> {
//     posts::table
//     .filter(posts::user_id.eq(user_id))
//     .get_result(conn)
// }