use actix_web::{dev::ServiceRequest, HttpMessage};

use super::jwt::decode_user;

pub fn auth_middleware(req: ServiceRequest) -> ServiceRequest {
    let unprotected_routes = vec!["/users/register", "/users/login", "/healthcheck"];
    if unprotected_routes.contains(&req.path()) {
        return req;
    }

    let token_option = req.headers().get("Authorization");
    let user = match token_option {
        Some(token) => decode_user(token.to_str().unwrap().to_string()),
        None => panic!("Unauthorized!")
    };
    req.extensions_mut().insert(user);

    req
}