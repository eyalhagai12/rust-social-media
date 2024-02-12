use diesel::{
    deserialize::Queryable, prelude::Insertable, query_dsl::methods::FilterDsl, ExpressionMethods, PgConnection, QueryResult, RunQueryDsl, Selectable, SelectableHelper
};
use serde::{Deserialize, Serialize};

use crate::{database::schema::{self, users::{self, password, username}}, schemas::{login_schema::UserLoginSchema, new_user::NewUserSchema}};

#[derive(Queryable, Selectable, Debug, Serialize, Deserialize)]
#[diesel(table_name = crate::database::schema::users)]
pub struct User {
    pub id: i32,
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::database::schema::users)]
pub struct NewUser {
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}

pub fn create_new_user(conn: &mut PgConnection, new_user_schema: NewUserSchema) -> QueryResult<User> {
    let new_user = NewUser {
        display_name: new_user_schema.display_name,
        username: new_user_schema.username,
        email: new_user_schema.email,
        password: new_user_schema.password,
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(conn)
}

pub fn get_user_with_credentials(conn: &mut PgConnection, user_login_schema: UserLoginSchema) -> QueryResult<User> {
    schema::users::table
    .filter(username.eq(user_login_schema.username))
    .filter(password.eq(user_login_schema.password))
    .get_result(conn)
}