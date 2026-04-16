use std::panic::Location;

use serde::Serialize;

#[macro_use]
mod macros;

pub type Result<T> = std::result::Result<T, AppError>;

/// Application error types
#[derive(Debug, PartialEq, Serialize)]
#[serde(
    tag = "error",
    content = "details",
    rename_all = "SCREAMING_SNAKE_CASE"
)]
pub enum ErrorType {
    FileTooLarge { max_size: usize, received: usize },

    InternalError,
}

/// Application Error data
#[derive(Debug, PartialEq)]
pub struct AppError {
    error_type: ErrorType,
    location: &'static Location<'static>,
}

impl AppError {
    /// Creates a new [`AppError`]
    #[track_caller]
    pub fn new(error_type: ErrorType) -> Self {
        Self {
            error_type,
            location: Location::caller(),
        }
    }

    /// Returns [`ErrorType`] of this [`AppError`]
    pub fn error_type(&self) -> &ErrorType {
        &self.error_type
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} occured in {}", self.error_type, self.location)
    }
}

impl std::error::Error for AppError {}

map_error!(std::io::Error => InternalError);
map_error!(sqlx::Error => InternalError);
map_error!(image::ImageError => InternalError);
