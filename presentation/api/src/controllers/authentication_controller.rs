use actix_web::{post, web, Responder, Result};
use application::authentication::commands::RegisterCommand;
use application::authentication::queries::LoginQuery;
use contracts::authentication::{AuthenticationResponse, LoginRequest, RegisterRequest};
use domain::common::errors::error_codes;

use crate::errors::app_error_response::AppErrorResponse;
use crate::ServicesInjected;

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/auth").service(register).service(login));
}

#[post("/register")]
async fn register(
    services: web::Data<ServicesInjected>,
    register_request: web::Json<RegisterRequest>,
) -> Result<impl Responder> {
    let register_command = RegisterCommand {
        first_name: register_request.0.first_name,
        last_name: register_request.0.last_name,
        email: register_request.0.email,
        password: register_request.0.password,
    };
    let auth_result = services
        .authentication_command
        .handle(register_command.into())
        .map_err(|err| {
            AppErrorResponse::from(err).add_error_code(error_codes::USER_DUPLICATE_EMAIL)
        })?;
    let response = AuthenticationResponse::from(auth_result);

    Ok(web::Json(response))
}

#[post("/login")]
async fn login(
    services: web::Data<ServicesInjected>,
    login_request: web::Json<LoginRequest>,
) -> Result<impl Responder> {
    let login_query = LoginQuery {
        email: login_request.0.email,
        password: login_request.0.password,
    };
    let auth_result = services
        .authentication_query
        .handle(login_query.into())
        .map_err(AppErrorResponse::from)?;
    let response = AuthenticationResponse::from(auth_result);

    Ok(web::Json(response))
}
