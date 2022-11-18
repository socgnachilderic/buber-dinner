use std::sync::Arc;

use application::authentication::commands::AuthenticationCommandHandler;
use application::authentication::queries::AuthenticationQueryHandler;
use application::common::interfaces::IJwtTokenManager;
use infrastructure::authentication::jwt_settings::JwtSettings;
use infrastructure::authentication::jwt_token_manager::JwtTokenManager;
use infrastructure::persistence::user_inmemory_repository::UserInMemoryRepository;
use infrastructure::services::date_provider::DateTimeProvider;

pub(crate) struct ServicesInjected {
    pub authentication_command: AuthenticationCommandHandler,
    pub authentication_query: AuthenticationQueryHandler,
    pub jwt_manager: Arc<dyn IJwtTokenManager>,
}

impl Default for ServicesInjected {
    fn default() -> Self {
        let env = super::env::Env::new();
        let jwt_settings = JwtSettings::new(
            env.jwt_secret,
            env.jwt_expiry_minutes,
            env.jwt_issuer,
            env.jwt_audience,
        );

        let datetime_provider = Arc::new(DateTimeProvider);
        let jwt_manager = Arc::new(JwtTokenManager::new(jwt_settings, datetime_provider));
        let user_repository = Arc::new(UserInMemoryRepository);

        Self {
            jwt_manager: jwt_manager.clone(),
            authentication_command: AuthenticationCommandHandler::new(
                jwt_manager.clone(),
                user_repository.clone(),
            ),
            authentication_query: AuthenticationQueryHandler::new(jwt_manager, user_repository),
        }
    }
}
