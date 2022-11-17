use actix_web::http::StatusCode;
use contracts::HTTP_STATUSES;
use serde::Deserialize;

pub use app_error::AppError;
pub use problem::Problem;
pub use validation_problem::ValidationProblem;

mod app_error;
mod problem;
mod validation_problem;

fn get_http_status_code(status: StatusCode) -> (u16, HttpStatus) {
    let status = status.as_u16();
    let http_status_codes: Vec<HttpStatus> = serde_json::from_str(HTTP_STATUSES).unwrap();

    let http_status = http_status_codes
        .into_iter()
        .find(|http_status| http_status.status_code == status.to_string())
        .unwrap();

    (status, http_status)
}

#[derive(Debug, Deserialize)]
struct HttpStatus {
    status_code: String,
    // reason_phrase: String,
    // one_liner: String,
    defined_in: String,
}
