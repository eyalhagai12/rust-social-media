use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HealthcheckResponse {
    pub status: String,
}
