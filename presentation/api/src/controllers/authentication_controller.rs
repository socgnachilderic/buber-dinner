use actix_web::{error, post, web, Responder, Result};
use application::services::authentication::IAuthenticationService;
use contracts::authentication::{AuthenticationResponse, LoginRequest, RegisterRequest};

use crate::ServicesInjected;

#[post("/register")]
async fn register(
    services: web::Data<ServicesInjected>,
    register_request: web::Json<RegisterRequest>,
) -> Result<impl Responder> {
    let auth_result = services
        .authentication
        .register(
            &register_request.0.first_name,
            &register_request.0.last_name,
            &register_request.0.email,
            &register_request.0.password,
        )
        .map_err(error::ErrorBadRequest)?;
    let response = AuthenticationResponse::from(auth_result);

    Ok(web::Json(response))
}

#[post("/login")]
async fn login(
    services: web::Data<ServicesInjected>,
    login_request: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let auth_result = services
        .authentication
        .login(&login_request.0.email, &login_request.0.password)
        .map_err(error::ErrorBadRequest)?;
    let response = AuthenticationResponse::from(auth_result);

    Ok(web::Json(response))
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register).service(login));
}
