use serde::Deserialize;

#[derive(Deserialize)]
pub struct UserLoginSchema {
    pub username: String,
    pub password: String,
}
