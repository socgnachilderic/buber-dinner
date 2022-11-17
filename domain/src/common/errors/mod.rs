use std::fmt;

pub use authentication_error::AuthenticationError;
pub use validation_error::ValidationError;

mod authentication_error;
pub mod error_codes;
mod validation_error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    Authentication(AuthenticationError),
    Validation(ValidationError),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Authentication(msg) => write!(f, "{msg}"),
            Error::Validation(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<AuthenticationError> for Error {
    fn from(error: AuthenticationError) -> Self {
        Error::Authentication(error)
    }
}

impl From<ValidationError> for Error {
    fn from(error: ValidationError) -> Self {
        Error::Validation(error)
    }
}
