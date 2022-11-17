use actix_web::http::StatusCode;
use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct Problem {
    #[serde(rename = "type")]
    kind: String,
    title: String,
    pub(super) status: u16,
    #[serde(rename = "errorCodes", skip_serializing_if = "Vec::is_empty")]
    pub error_codes: Vec<String>,
}

impl Problem {
    pub fn new(error: impl ToString, status_code: StatusCode) -> Self {
        let (status, http_status) = super::get_http_status_code(status_code);

        Self {
            status,
            title: error.to_string(),
            kind: http_status.defined_in,
            error_codes: Vec::new(),
        }
    }

    pub fn add_error_code(mut self, error_code: &str) -> Self {
        self.error_codes.push(error_code.to_string());
        self
    }

    pub(super) fn set_status(mut self, status_code: StatusCode) -> Self {
        self.status = status_code.as_u16();
        self
    }
}
