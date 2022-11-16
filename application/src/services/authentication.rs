use domain::common::errors::AuthenticationError;
use domain::entities::user::User;
use domain::Result;

use crate::common::interfaces::authentication::IJwtTokenGenerator;
use crate::persistence::user_repository::IUserRepository;

pub trait IAuthenticationService {
    fn register(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthenticationResult>;

    fn login(&self, email: &str, password: &str) -> Result<AuthenticationResult>;
}

pub struct AuthenticationService {
    jwt_token_generator: Box<dyn IJwtTokenGenerator>,
    user_repository: Box<dyn IUserRepository>,
}

impl AuthenticationService {
    pub fn new(
        jwt_token_generator: Box<dyn IJwtTokenGenerator>,
        user_repository: Box<dyn IUserRepository>,
    ) -> Self {
        Self {
            jwt_token_generator,
            user_repository,
        }
    }

    fn get_user_with_token(&self, user: User) -> AuthenticationResult {
        let token = self.jwt_token_generator.generate_token(&user);

        AuthenticationResult { user, token }
    }
}

impl IAuthenticationService for AuthenticationService {
    fn register(
        &self,
        first_name: &str,
        last_name: &str,
        email: &str,
        password: &str,
    ) -> Result<AuthenticationResult> {
        if self.user_repository.get_user_by_email(email).is_some() {
            return Err(AuthenticationError::EmailAlreadyExist.into());
        }

        let user = User::new(first_name, last_name, email, password);

        self.user_repository.add(&user);
        Ok(self.get_user_with_token(user))
    }

    fn login(&self, email: &str, password: &str) -> Result<AuthenticationResult> {
        if let Some(user) = self.user_repository.get_user_by_email(email) {
            if user.password == password {
                return Ok(self.get_user_with_token(user));
            }
        }

        Err(AuthenticationError::InvalidCredentials.into())
    }
}

pub struct AuthenticationResult {
    pub user: User,
    pub token: String,
}
