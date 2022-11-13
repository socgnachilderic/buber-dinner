use domain::entities::user::User;

pub trait IUserRepository {
    fn get_user_by_email(&self, email: &str) -> Option<User>;
    fn add(&self, user: &User);
}
