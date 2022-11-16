// use thiserror::Error as ThisError;

pub mod common;
pub mod persistence;
pub mod services;

// #[derive(Debug, ThisError)]
// pub enum Error {
//     #[error(transparent)]
//     AuthenticationError(#[from] services::authentication::AuthenticationError),
// }

// pub type Result<T> = std::result::Result<T, Error>;
