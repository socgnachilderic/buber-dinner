use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use domain::common::errors::{AuthenticationError, Error, ValidationError};
use serde::Serialize;

use super::{Problem, ValidationProblem};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AppError {
    Problem(Problem),
    Validation(ValidationProblem),
}

impl ResponseError for AppError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        let status = match self {
            AppError::Problem(problem) => problem.status,
            AppError::Validation(problem) => problem.status,
        };

        StatusCode::from_u16(status).unwrap()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.status_code())
            .append_header(("content-type", "application/problem+json; charset=utf-8"))
            .json(self)
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = match self {
            AppError::Problem(problem) => serde_json::to_string(problem).unwrap(),
            AppError::Validation(problem) => serde_json::to_string(problem).unwrap(),
        };

        write!(f, "{value}")
    }
}

impl From<Problem> for AppError {
    fn from(problem: Problem) -> Self {
        AppError::Problem(problem)
    }
}

impl From<ValidationProblem> for AppError {
    fn from(problem: ValidationProblem) -> Self {
        AppError::Validation(problem)
    }
}

impl From<Error> for AppError {
    fn from(error: Error) -> Self {
        match error {
            Error::Authentication(error) => error.into(),
            Error::Validation(error) => error.into(),
        }
    }
}

impl From<AuthenticationError> for AppError {
    fn from(error: AuthenticationError) -> Self {
        let err = Problem::new(&error, StatusCode::INTERNAL_SERVER_ERROR);
        let error = match error {
            AuthenticationError::EmailAlreadyExist => err.set_status(StatusCode::CONFLICT),
            AuthenticationError::InvalidCredentials => err.set_status(StatusCode::UNAUTHORIZED),
        };

        error.into()
    }
}

impl From<ValidationError> for AppError {
    fn from(error: ValidationError) -> Self {
        let mut problem = ValidationProblem::new(error.error_message, StatusCode::BAD_REQUEST);

        problem.errors = error.errors;
        problem.into()
    }
}
