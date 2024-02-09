use actix_web::{HttpResponse, Responder};
use log::debug;

use crate::{logging, responses};

pub async fn healthcheck() -> impl Responder {
    let logger = logging::Logger::new("Healthcheck");

    let response = responses::healthcheck::HealthcheckResponse {
        status: "Healthy".to_string(),
    };

    logger.info("Service is healthy").await.expect("failed to log healthcheck");

    debug!("Service Status: {}", response.status);
    HttpResponse::Ok().json(response)
}
