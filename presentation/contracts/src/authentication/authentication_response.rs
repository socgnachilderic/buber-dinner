use application::services::authentication::AuthenticationResult;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthenticationResponse {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub token: String,
}

impl From<AuthenticationResult> for AuthenticationResponse {
    fn from(auth_result: AuthenticationResult) -> Self {
        Self {
            id: auth_result.id.to_string(),
            first_name: auth_result.first_name,
            last_name: auth_result.last_name,
            email: auth_result.email,
            token: auth_result.token,
        }
    }
}
