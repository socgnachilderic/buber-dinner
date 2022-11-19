use domain::user::User;

pub mod commands;
pub mod queries;

pub struct AuthenticationResult {
    pub user: User,
    pub token: String,
}
