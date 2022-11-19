use std::sync::{Arc, Mutex};

use application::common::interfaces::IUserRepository;
use domain::user::User;
use lazy_static::lazy_static;
use uuid::Uuid;

lazy_static! {
    static ref USER_MEMORY: Arc<Mutex<Vec<MemoryUser>>> = Arc::new(Mutex::new(Vec::new()));
}

pub struct UserInMemoryRepository;

impl IUserRepository for UserInMemoryRepository {
    fn get_user_by_email(&self, email: &str) -> Option<User> {
        USER_MEMORY
            .lock()
            .unwrap()
            .iter()
            .find(|user| user.email == email)
            .map(|user| User {
                id: user.id,
                first_name: user.first_name.to_string(),
                last_name: user.last_name.to_string(),
                email: user.email.to_string(),
                password: user.password.to_string(),
            })
    }

    fn add(&self, user: &User) {
        let user: MemoryUser = MemoryUser {
            id: user.id.to_owned(),
            first_name: user.first_name.to_owned(),
            last_name: user.last_name.to_owned(),
            email: user.email.to_owned(),
            password: user.password.to_owned(),
        };

        USER_MEMORY.lock().unwrap().push(user);
    }
}

struct MemoryUser {
    id: Uuid,
    first_name: String,
    last_name: String,
    email: String,
    password: String,
}
