#[derive(Debug)]
pub(super) struct Env {
    pub jwt_secret: String,
    pub jwt_issuer: String,
    pub jwt_audience: String,
    pub jwt_expiry_minutes: u64,
}

impl Env {
    pub fn new() -> Self {
        dotenvy::dotenv().ok();

        Self {
            jwt_secret: Self::get_var("JWT_SECRET"),
            jwt_issuer: Self::get_var("JWT_ISSUER"),
            jwt_audience: Self::get_var("JWT_AUDIENCE"),
            jwt_expiry_minutes: Self::get_var("JWT_EXPIRY_MINUTES").parse::<u64>().unwrap(),
        }
    }

    fn get_var(key: &str) -> String {
        std::env::var(key).unwrap_or_else(|_| panic!("{key} not found."))
    }
}
