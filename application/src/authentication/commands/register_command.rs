use domain::common::errors::*;
use domain::entities::user::User;
use serde::{Deserialize, Serialize};

use crate::authentication::AuthenticationResult;

use super::AuthenticationCommandHandler;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterCommand {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl RegisterCommand {
    pub(super) fn handle(
        self,
        handler: &AuthenticationCommandHandler,
    ) -> Result<AuthenticationResult> {
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
