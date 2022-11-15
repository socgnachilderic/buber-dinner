use std::error::Error;
use std::fmt;

use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppErrorResponse {
    #[serde(rename = "type")]
    kind: String,
    title: String,
    status: u16,
}

impl AppErrorResponse {
    pub fn from_error(error: impl Error, status_code: StatusCode) -> Self {
        let status = status_code.as_u16();
        let http_status = get_http_status_code(status);

        Self {
            status,
            title: error.to_string(),
            kind: http_status.defined_in,
        }
    }

    fn get_status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap()
    }
}

impl Error for AppErrorResponse {}

impl fmt::Display for AppErrorResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let value = serde_json::to_string(self).unwrap();

        write!(f, "{value}")
    }
}

impl ResponseError for AppErrorResponse {
    fn status_code(&self) -> StatusCode {
        self.get_status_code()
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        HttpResponse::build(self.get_status_code())
            .append_header(("content-type", "application/problem+json; charset=utf-8"))
            .json(self)
    }
}

fn get_http_status_code(status: u16) -> HttpStatus {
    let http_status_codes: Vec<HttpStatus> = serde_json::from_str(super::HTTP_STATUSES).unwrap();

    http_status_codes
        .into_iter()
        .find(|http_status| http_status.status_code == status.to_string())
        .unwrap()
}

#[derive(Debug, Deserialize)]
struct HttpStatus {
    status_code: String,
    // reason_phrase: String,
    // one_liner: String,
    defined_in: String,
}
