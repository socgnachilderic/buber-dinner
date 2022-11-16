pub mod common;
pub mod entities;

pub use common::errors::Error;

pub type Result<T> = std::result::Result<T, Error>;
