use ms_shared_models::domains::{AssetError, MediaError};
use serde::Serialize;

#[macro_use]
pub mod macros;

pub mod api;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "error", content = "reason")]
pub enum AppError {
    AssetError(AssetError),
    MediaError(MediaError),

    InternalError,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

map_error!(AssetError => AppError::AssetError);
map_error!(MediaError => AppError::MediaError);

// TODO: Fix this mappers

use ms_content_store::ContentStorageError;
map_error! {
    ContentStorageError => |e| match e {
        ContentStorageError::BlobTooLarge => AppError::MediaError(MediaError::MediaTooLarge),
        ContentStorageError::InvalidStorageKey => AppError::MediaError(MediaError::BadMediaKey),
        _ => AppError::InternalError,
    }
}

map_error!(ms_blob_host::BlobHostError => InternalError);

map_error!(ms_database::DatabaseError => InternalError);
map_error!(ms_media::MediaProcessingError => InternalError);

map_error!(std::io::Error => InternalError);
map_error!(actix_multipart::MultipartError => InternalError);
