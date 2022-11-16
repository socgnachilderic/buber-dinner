use std::fmt;

pub use authentication_error::AuthenticationError;

mod authentication_error;
pub mod error_codes;

#[derive(Debug)]
pub enum Error {
    Authentication(AuthenticationError),
}

impl std::error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Authentication(msg) => write!(f, "{msg}"),
        }
    }
}

impl From<AuthenticationError> for Error {
    fn from(error: AuthenticationError) -> Self {
        Error::Authentication(error)
    }
}
