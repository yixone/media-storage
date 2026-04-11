use std::panic::Location;

type ErrorSourceDyn = Box<dyn std::error::Error + Send + Sync + 'static>;

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
}

/// Application error types
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    InternalError,
}
