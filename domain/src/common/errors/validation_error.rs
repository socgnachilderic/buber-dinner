use std::collections::HashMap;
use std::fmt;

const VALIDATION_MESSAGE_ERROR: &str = "One or more validation errors occurred.";

#[derive(Debug)]
pub struct ValidationError {
    pub errors: HashMap<String, Vec<String>>,
    pub error_message: String,
}

impl ValidationError {
    pub fn add_error(mut self, field: &str, messages: Vec<String>) -> Self {
        self.errors.insert(field.to_string(), messages);
        self
    }
}

impl Default for ValidationError {
    fn default() -> Self {
        Self {
            errors: Default::default(),
            error_message: VALIDATION_MESSAGE_ERROR.to_string(),
        }
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.error_message)
    }
}
