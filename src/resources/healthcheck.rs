use actix_web::{HttpResponse, Responder};

use crate::responses;

pub async fn healthcheck() -> impl Responder {
    let response = responses::healthcheck::HealthcheckResponse {
        status: "Healthy".to_string(),
    };
    HttpResponse::Ok().json(response)
}
