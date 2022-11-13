use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: String,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
}

impl User {
    pub fn new(first_name: &str, last_name: &str, email: &str, password: &str) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            first_name: first_name.to_lowercase(),
            last_name: last_name.to_lowercase(),
            email: email.to_lowercase(),
            password: password.to_string(),
        }
    }
}
