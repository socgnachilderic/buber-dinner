use std::fmt;

const EMAIL_ALREADY_EXIST_MESSAGE: &str = "Email is already in use.";
const INVALID_CREDENTIALS_MESSAGE: &str = "Invalid credentials.";

#[derive(Debug)]
pub enum AuthenticationError {
    EmailAlreadyExist,
    InvalidCredentials,
}

impl fmt::Display for AuthenticationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AuthenticationError::EmailAlreadyExist => write!(f, "{EMAIL_ALREADY_EXIST_MESSAGE}"),
            AuthenticationError::InvalidCredentials => write!(f, "{INVALID_CREDENTIALS_MESSAGE}"),
        }
    }
}
