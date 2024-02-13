use diesel::{Queryable, Selectable};
use serde::Serialize;

use crate::database::schema::posts;

#[derive(Queryable, Selectable, Debug, Serialize)]
#[diesel(table_name = posts)]
pub struct Post {
    pub id: i32,
    pub description: String,
    pub user_id: i32,
}
