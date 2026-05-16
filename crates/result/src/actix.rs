use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::{AppError, error::ErrorKind};

/// DTO for API server error
#[derive(Serialize)]
pub struct ErrorResponse<'a> {
    /// HTTP error code
    pub code: u16,

    /// Kind of error
    #[serde(flatten)]
    pub error: &'a ErrorKind,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.kind {
            ErrorKind::AssetDeleted => StatusCode::FORBIDDEN,
            ErrorKind::MissingAssetMedia => StatusCode::INTERNAL_SERVER_ERROR,

            ErrorKind::MediaTooLarge { .. } => StatusCode::PAYLOAD_TOO_LARGE,
            ErrorKind::UnsupportedMediaType => StatusCode::BAD_REQUEST,
            ErrorKind::MissingUploadMedia => StatusCode::BAD_REQUEST,
            ErrorKind::MissingMediaBlob => StatusCode::INTERNAL_SERVER_ERROR,

            ErrorKind::NotFound => StatusCode::NOT_FOUND,

            ErrorKind::Internal { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();

        if status.is_server_error() {
            tracing::error!(
                error = ?self.kind,
                file = self.location.file(),
                line = self.location.line()
            );
        }

        let res = ErrorResponse {
            code: status.as_u16(),
            error: &self.kind,
        };

        HttpResponse::build(status).json(res)
    }
}
