use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::error::AppError;

use super::ErrorType;

/// DTO for API error
#[derive(Serialize)]
struct ErrorResponse<'a> {
    /// Http status code
    code: u16,
    /// Error information
    error: &'a ErrorType,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type() {
            ErrorType::FileTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            ErrorType::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::MediaError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let code = status.as_u16();
        HttpResponse::build(status).json(ErrorResponse {
            code,
            error: self.error_type(),
        })
    }
}
