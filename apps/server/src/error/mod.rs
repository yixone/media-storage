use serde::Serialize;

#[macro_use]
pub mod macros;

pub mod api;

pub type AppResult<T> = std::result::Result<T, AppError>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AppError {
    InvalidId,
    FileTooLarge,

    AlreadyExists,
    NotFound,

    InternalError,
    MultipartError,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for AppError {}

map_error! {
    ms_database::DatabaseError => |e| {
        match e {
            ms_database::DatabaseError::NotFound => AppError::NotFound,
            ms_database::DatabaseError::Conflict => AppError::AlreadyExists,
            _ => AppError::InternalError,
        }
    }
}

map_error! {
    ms_content_store::ContentStorageError => |e| {
        match e {
            ms_content_store::ContentStorageError::BlobTooLarge => AppError::FileTooLarge,
            ms_content_store::ContentStorageError::InvalidStorageKey => AppError::InvalidId,
            _ => AppError::InternalError,
        }
    }
}

map_error! {
    ms_blob_host::BlobHostError => |e| {
        match e {
            ms_blob_host::BlobHostError::InvalidBlobPath => AppError::InvalidId,
            ms_blob_host::BlobHostError::BlobPathConflict => AppError::AlreadyExists,
            _ => AppError::InternalError,
        }
    }
}

map_error!(std::io::Error => InternalError);
map_error!(actix_multipart::MultipartError => MultipartError);
