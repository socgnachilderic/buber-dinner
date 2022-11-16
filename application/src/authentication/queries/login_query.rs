use domain::common::errors::*;
use serde::{Deserialize, Serialize};

use crate::authentication::AuthenticationResult;

use super::AuthenticationQueryHandler;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginQuery {
    pub email: String,
    pub password: String,
}

impl LoginQuery {
    pub(super) fn handle(
        self,
        handler: &AuthenticationQueryHandler,
    ) -> Result<AuthenticationResult> {
        if let Some(user) = handler.user_repository.get_user_by_email(&self.email) {
            if user.password == self.password {
                let token = handler.jwt_token_generator.generate_token(&user);

                return Ok(AuthenticationResult { user, token });
            }
        }

        Err(AuthenticationError::InvalidCredentials.into())
    }
}
