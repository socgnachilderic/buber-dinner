use actix_web::dev::ServiceRequest;
use actix_web::{web, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;

use dependency_injection::ServicesInjected;

pub mod dependency_injection;
pub mod env;

pub async fn http_bearer_authentication(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let config = req.app_data::<Config>().cloned().unwrap_or_default();
    let injector = req.app_data::<web::Data<ServicesInjected>>().unwrap();

    if let Some(id) = injector.jwt_manager.validate_token(credentials.token()) {
        req.extensions_mut().insert(id);
        Ok(req)
    } else {
        Err((AuthenticationError::from(config).into(), req))
    }
}
