use std::collections::HashMap;

use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct ValidationProblem {
    #[serde(rename = "type")]
    kind: String,
    title: String,
    pub(super) status: u16,
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    pub errors: HashMap<String, Vec<String>>,
}

impl ValidationProblem {
    pub fn new(error: impl ToString, status_code: StatusCode) -> Self {
        let (status, http_status) = super::get_http_status_code(status_code);

        Self {
            status,
            title: error.to_string(),
            kind: http_status.defined_in,
            errors: HashMap::new(),
        }
    }
}
