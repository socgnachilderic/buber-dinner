use domain::entities::user::User;

pub trait IJwtTokenGenerator {
    fn generate_token(&self, user: &User) -> String;
}

pub trait IUserRepository {
    fn get_user_by_email(&self, email: &str) -> Option<User>;
    fn add(&self, user: &User);
}

pub trait IDateProvider {
    fn now(&self) -> u64;
}
