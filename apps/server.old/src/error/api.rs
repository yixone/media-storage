use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;

use crate::error::AppError;

#[derive(Serialize)]
pub struct ApiError<'a> {
    code: u16,
    #[serde(flatten)]
    error: &'a AppError,
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::AssetError(e) => match e {
                shelf_shared_models::domains::AssetError::MissingUploadMedia => {
                    StatusCode::BAD_REQUEST
                }
                shelf_shared_models::domains::AssetError::InvalidSourceUrl => {
                    StatusCode::BAD_REQUEST
                }
                shelf_shared_models::domains::AssetError::AssetNotFound => StatusCode::NOT_FOUND,
            },
            AppError::MediaError(e) => match e {
                shelf_shared_models::domains::MediaError::InvalidMimetype => {
                    StatusCode::BAD_REQUEST
                }
                shelf_shared_models::domains::MediaError::MediaTooLarge => {
                    StatusCode::PAYLOAD_TOO_LARGE
                }
                shelf_shared_models::domains::MediaError::BadMediaKey => StatusCode::BAD_REQUEST,
                shelf_shared_models::domains::MediaError::MediaNotFound => StatusCode::NOT_FOUND,
            },
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let code = status.as_u16();

        HttpResponse::build(status).json(ApiError { code, error: self })
    }
}
