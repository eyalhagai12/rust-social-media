use std::fmt::Display;

use actix_web::cookie::time::Time;
use diesel::{deserialize::Queryable, Selectable};

use crate::database::schema::posts;

#[derive(Selectable, Queryable, Debug)]
#[diesel(table_name = posts)]
struct Post {
    pub id: i32,
    pub description: String,
    pub user_id: i32,
    pub posting_date: Time,
}
