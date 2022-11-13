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
            id: auth_result.user.id,
            first_name: auth_result.user.first_name,
            last_name: auth_result.user.last_name,
            email: auth_result.user.email,
            token: auth_result.token,
        }
    }
}
