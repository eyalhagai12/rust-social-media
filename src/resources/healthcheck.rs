use actix_web::{HttpResponse, Responder};
use log::debug;

use crate::responses;

pub async fn healthcheck() -> impl Responder {
    let response = responses::healthcheck::HealthcheckResponse {
        status: "Healthy".to_string(),
    };

    debug!("Service Status: {}", response.status);
    HttpResponse::Ok().json(response)
}
