use domain::entities::user::User;

pub trait IJwtTokenGenerator {
    fn generate_token(&self, user: &User) -> String;
}
