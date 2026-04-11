type ErrorSourceDyn = Box<dyn std::error::Error + Send + Sync + 'static>;

/// Application Error data
#[derive(Debug)]
pub struct AppError {
    error_type: ErrorType,
    location: String,
    source: Option<ErrorSourceDyn>,
}

/// Application error types
#[derive(Debug, PartialEq)]
pub enum ErrorType {
    InternalError,
}
