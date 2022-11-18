use actix_web::{post, web, HttpResponse, Responder, Result};
use application::authentication::commands::RegisterCommand;
use application::authentication::queries::LoginQuery;
use contracts::authentication::{AuthenticationResponse, LoginRequest, RegisterRequest};
use domain::common::errors::error_codes;

use crate::errors::AppError;
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
            let mut error = AppError::from(err);

            if let AppError::Problem(problem) = &error {
                if problem.error_codes.is_empty() {
                    error = problem
                        .clone()
                        .add_error_code(error_codes::USER_DUPLICATE_EMAIL)
                        .into();
                }
            }

            error
        })?;
    let response = AuthenticationResponse::from(auth_result);

    Ok(HttpResponse::Created().json(response))
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
        .map_err(AppError::from)?;
    let response = AuthenticationResponse::from(auth_result);

    Ok(HttpResponse::Ok().json(response))
}
