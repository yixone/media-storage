use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::error::AppError;

#[derive(Serialize)]
pub struct ApiError<'a> {
    code: u16,
    error: &'a AppError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::InvalidId => StatusCode::BAD_REQUEST,
            AppError::FileTooLarge => StatusCode::PAYLOAD_TOO_LARGE,
            AppError::AlreadyExists => StatusCode::CONFLICT,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let code = status.as_u16();

        HttpResponse::build(status).json(ApiError { code, error: self })
    }
}
