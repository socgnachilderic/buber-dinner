use std::sync::Arc;

use domain::Result;
use serde::{Deserialize, Serialize};

pub use register_command::RegisterCommand;

use crate::common::interfaces::{IJwtTokenGenerator, IUserRepository};

mod register_command;

use super::AuthenticationResult;

pub struct AuthenticationCommandHandler {
    jwt_token_generator: Arc<dyn IJwtTokenGenerator>,
    user_repository: Arc<dyn IUserRepository>,
}

impl AuthenticationCommandHandler {
    pub fn new(
        jwt_token_generator: Arc<dyn IJwtTokenGenerator>,
        user_repository: Arc<dyn IUserRepository>,
    ) -> Self {
        Self {
            jwt_token_generator,
            user_repository,
        }
    }

    pub fn handle(&self, command: AuthenticationCommand) -> Result<AuthenticationResult> {
        match command {
            AuthenticationCommand::Register(command) => command.handle(self),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationCommand {
    Register(RegisterCommand),
}

impl From<RegisterCommand> for AuthenticationCommand {
    fn from(command: RegisterCommand) -> Self {
        Self::Register(command)
    }
}
