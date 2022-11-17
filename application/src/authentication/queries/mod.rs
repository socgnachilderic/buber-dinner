use std::sync::Arc;

use domain::common::errors::Result;
use serde::{Deserialize, Serialize};

use crate::common::interfaces::{IJwtTokenGenerator, IUserRepository, ItemHandle};

pub use login_query::LoginQuery;

use super::AuthenticationResult;

mod login_query;

pub struct AuthenticationQueryHandler {
    jwt_token_generator: Arc<dyn IJwtTokenGenerator>,
    user_repository: Arc<dyn IUserRepository>,
}

impl AuthenticationQueryHandler {
    pub fn new(
        jwt_token_generator: Arc<dyn IJwtTokenGenerator>,
        user_repository: Arc<dyn IUserRepository>,
    ) -> Self {
        Self {
            jwt_token_generator,
            user_repository,
        }
    }

    pub fn handle(&self, query: AuthenticationQuery) -> Result<AuthenticationResult> {
        match query {
            AuthenticationQuery::Login(query) => query.handle(self),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AuthenticationQuery {
    Login(LoginQuery),
}

impl From<LoginQuery> for AuthenticationQuery {
    fn from(query: LoginQuery) -> Self {
        Self::Login(query)
    }
}
