use std::sync::Arc;

use application::common::interfaces::{IDateProvider, IJwtTokenManager};
use domain::user::User;
use domain::common::UID;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

use super::jwt_settings::JwtSettings;

const MINUTES: u64 = 60;

pub struct JwtTokenManager {
    jwt_settings: JwtSettings,
    datetime_provider: Arc<dyn IDateProvider>,
}

impl JwtTokenManager {
    pub fn new(jwt_settings: JwtSettings, datetime_provider: Arc<dyn IDateProvider>) -> Self {
        Self {
            jwt_settings,
            datetime_provider,
        }
    }
}

impl IJwtTokenManager for JwtTokenManager {
    fn generate_token(&self, user: &User) -> String {
        let jwt_header = Header::default();
        let secret_key = EncodingKey::from_secret(self.jwt_settings.secret.as_bytes());
        let now = self.datetime_provider.now();
        let exp_date = now + self.jwt_settings.expiry_minutes * MINUTES;
        let first_name = &user.first_name;
        let last_name = &user.last_name;

        let claims = Claims {
            iss: self.jwt_settings.issuer.to_owned(),
            aud: self.jwt_settings.audience.to_owned(),
            sub: format!("{first_name} {last_name}"),
            jti: user.id,
            given_name: first_name.to_string(),
            family_name: last_name.to_string(),
            iat: now,
            exp: exp_date,
        };

        encode(&jwt_header, &claims, &secret_key).unwrap()
    }

    fn validate_token(&self, token: &str) -> Option<UID> {
        let secret_key = DecodingKey::from_secret(self.jwt_settings.secret.as_bytes());

        decode::<Claims>(token, &secret_key, &Validation::default())
            .map(|token_data| token_data.claims.jti)
            .ok()
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Claims {
    sub: String,
    iss: String,
    aud: String,
    jti: UID,
    given_name: String,
    family_name: String,
    iat: u64,
    exp: u64,
}
