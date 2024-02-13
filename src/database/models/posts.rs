use diesel::{
    ExpressionMethods, PgConnection, QueryDsl, QueryResult, Queryable, RunQueryDsl, Selectable,
};
use serde::Serialize;

use crate::database::schema::posts;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub description: String,
    pub user_id: i32,
}

pub fn get_all_user_posts(conn: &mut PgConnection, user_id: i32) -> QueryResult<Vec<Post>> {
    posts::table.filter(posts::user_id.eq(user_id)).load(conn)
}
