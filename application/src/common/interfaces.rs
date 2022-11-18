use domain::common::errors::{Error, Result, ValidationError};
use domain::entities::user::User;
use domain::entities::UID;
use inflections::case::{to_pascal_case, to_title_case};
use validator::Validate;

pub trait IJwtTokenManager {
    fn generate_token(&self, user: &User) -> String;
    fn validate_token(&self, token: &str) -> Option<UID>;
}

pub trait IUserRepository {
    fn get_user_by_email(&self, email: &str) -> Option<User>;
    fn add(&self, user: &User);
}

pub trait IDateProvider {
    fn now(&self) -> u64;
}

pub trait ItemHandle: Validate {
    type Item;
    type Handle;

    fn handle(&self, handler: &Self::Handle) -> Result<Self::Item> {
        self.validate().map_err(|error| {
            let error = error.field_errors().into_iter().fold(
                ValidationError::default(),
                |error, (field, errors)| {
                    let messages = errors
                        .iter()
                        .map(|err| format!("'{}' {}", to_title_case(field), err))
                        .collect();

                    error.add_error(&to_pascal_case(field), messages)
                },
            );

            Error::from(error)
        })?;

        self.service(handler)
    }

    fn service(&self, handler: &Self::Handle) -> Result<Self::Item>;
}
