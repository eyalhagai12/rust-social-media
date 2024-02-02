use serde::Serialize;

#[derive(Serialize)]
pub struct UserResponse {
    pub display_name: String,
    pub username: String,
    pub email: String,
}
