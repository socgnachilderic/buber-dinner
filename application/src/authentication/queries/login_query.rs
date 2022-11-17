use domain::common::errors::*;
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::authentication::AuthenticationResult;
use crate::common::interfaces::ItemHandle;

use super::AuthenticationQueryHandler;

#[derive(Debug, Serialize, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct LoginQuery {
    #[validate(length(min = 1, message = "must not be empty"))]
    pub email: String,
    #[validate(length(min = 1, message = "must not be empty"))]
    pub password: String,
}

impl ItemHandle for LoginQuery {
    type Item = AuthenticationResult;
    type Handle = AuthenticationQueryHandler;

    fn service(&self, handler: &Self::Handle) -> Result<Self::Item> {
        if let Some(user) = handler.user_repository.get_user_by_email(&self.email) {
            if user.password == self.password {
                let token = handler.jwt_token_generator.generate_token(&user);

                return Ok(AuthenticationResult { user, token });
            }
        }

        Err(AuthenticationError::InvalidCredentials.into())
    }
}
