#[derive(Debug, Default, Clone)]
pub struct JwtSettings {
    pub secret: String,
    pub expiry_minutes: u64,
    pub issuer: String,
    pub audience: String,
}

impl JwtSettings {
    pub fn new(secret: String, expiry_minutes: u64, issuer: String, audience: String) -> Self {
        Self {
            secret,
            expiry_minutes,
            issuer,
            audience,
        }
    }
}
