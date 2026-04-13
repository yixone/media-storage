use std::panic::Location;

#[macro_use]
mod macros;

pub type Result<T> = std::result::Result<T, AppError>;

type ErrorSourceDyn = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Application error types
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    InternalError,
}

/// Application Error data
#[derive(Debug)]
pub struct AppError {
    error_type: ErrorType,
    location: String,
    source: Option<ErrorSourceDyn>,
}

impl AppError {
    /// Creates a new [`AppError`]
    #[track_caller]
    pub fn new(error_type: ErrorType, source: Option<ErrorSourceDyn>) -> Self {
        let caller = Location::caller();
        let location = format!("{}:{}", caller.file(), caller.line());
        Self {
            error_type,
            location,
            source,
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

impl std::error::Error for AppError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_deref().map(|s| s as &dyn std::error::Error)
    }
}

map_error!(std::io::Error => InternalError);
