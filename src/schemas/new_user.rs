use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUserSchema {
    pub display_name: String,
    pub username: String,
    pub email: String,
    pub password: String,
}
