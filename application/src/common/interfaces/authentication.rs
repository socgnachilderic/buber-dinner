pub trait IJwtTokenGenerator {
    fn generate_token(&self, user_id: &str, first_name: &str, last_name: &str) -> String;
}
