use application::common::interfaces::{
    authentication::IJwtTokenGenerator, services::IDateProvider,
};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use super::jwt_settings::JwtSettings;

const MINUTES: u64 = 60;

pub struct JwtTokenGenerator {
    jwt_settings: JwtSettings,
    datetime_provider: Box<dyn IDateProvider>,
}

impl JwtTokenGenerator {
    pub fn new(jwt_settings: JwtSettings, datetime_provider: Box<dyn IDateProvider>) -> Self {
        Self {
            jwt_settings,
            datetime_provider,
        }
    }
}

impl IJwtTokenGenerator for JwtTokenGenerator {
    fn generate_token(&self, user_id: &str, first_name: &str, last_name: &str) -> String {
        let jwt_header = Header::default();
        let secret_key = EncodingKey::from_secret(self.jwt_settings.secret.as_bytes());
        let now = self.datetime_provider.now();
        let exp_date = now + self.jwt_settings.expiry_minutes * MINUTES;

        let claims = Claims {
            iss: self.jwt_settings.issuer.to_owned(),
            aud: self.jwt_settings.audience.to_owned(),
            sub: format!("{first_name} {last_name}"),
            jti: user_id.to_string(),
            given_name: first_name.to_string(),
            family_name: last_name.to_string(),
            iat: now,
            exp: exp_date,
        };

        encode(&jwt_header, &claims, &secret_key).unwrap()
    }
}

#[derive(Debug, Serialize, Deserialize)]
// #[serde(rename_all="camelCase")]
pub struct Claims {
    sub: String,
    iss: String,
    aud: String,
    jti: String,
    given_name: String,
    family_name: String,
    iat: u64,
    exp: u64,
}
