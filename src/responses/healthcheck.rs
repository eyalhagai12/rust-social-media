use serde::Serialize;

#[derive(Serialize)]
pub struct HealthcheckResponse {
    pub status: String,
}
