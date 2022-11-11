use crate::common::interfaces::authentication::IJwtTokenGenerator;
use uuid::Uuid;

pub trait IAuthenticationService {
    fn register(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        password: String,
    ) -> AuthenticationResult;

    fn login(email: String, password: String) -> AuthenticationResult;
}

pub struct AuthenticationService {
    jwt_token_generator: Box<dyn IJwtTokenGenerator>,
}

impl AuthenticationService {
    pub fn new(jwt_token_generator: Box<dyn IJwtTokenGenerator>) -> Self {
        Self {
            jwt_token_generator,
        }
    }
}

impl IAuthenticationService for AuthenticationService {
    fn register(
        &self,
        first_name: String,
        last_name: String,
        email: String,
        _password: String,
    ) -> AuthenticationResult {
        let id = Uuid::new_v4();
        let user_id = id.to_string();
        let token = self
            .jwt_token_generator
            .generate_token(&user_id, &first_name, &last_name);

        AuthenticationResult {
            id,
            first_name,
            last_name,
            email,
            token,
        }
    }

    fn login(email: String, _password: String) -> AuthenticationResult {
        AuthenticationResult {
            id: Uuid::new_v4(),
            first_name: "first_name".to_string(),
            last_name: "last_name".to_string(),
            email,
            token: "token".to_string(),
        }
    }
}

pub struct AuthenticationResult {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub token: String,
}
