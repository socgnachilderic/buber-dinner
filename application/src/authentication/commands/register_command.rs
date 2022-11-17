use domain::common::errors::*;
use domain::entities::user::User;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::authentication::AuthenticationResult;
use crate::common::interfaces::ItemHandle;

use super::AuthenticationCommandHandler;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCommand {
    #[validate(length(min = 1, message = "must not be empty"))]
    pub first_name: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub password: String,
}

impl ItemHandle for RegisterCommand {
    type Item = AuthenticationResult;
    type Handle = AuthenticationCommandHandler;

    fn service(&self, handler: &Self::Handle) -> Result<Self::Item> {
        if handler
            .user_repository
            .get_user_by_email(&self.email)
            .is_some()
        {
            return Err(AuthenticationError::EmailAlreadyExist.into());
        }

        let user = User::new(
            &self.first_name,
            &self.last_name,
            &self.email,
            &self.password,
        );
        let token = handler.jwt_token_generator.generate_token(&user);

        handler.user_repository.add(&user);
        Ok(AuthenticationResult { user, token })
    }
}
