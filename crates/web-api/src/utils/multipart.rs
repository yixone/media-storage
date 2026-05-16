use actix_multipart::Field;
use asset_shelf_result::{create_error, error::AppResult};
use futures::TryStreamExt;

#[derive(Debug)]
enum MultipartParseError {
    FieldReadingError,
    StringParsingError,
}

impl std::fmt::Display for MultipartParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for MultipartParseError {}

/// Reads a multipart [`Field`] into a [`String`]
pub async fn read_field_to_string(field: &mut Field) -> AppResult<String> {
    let mut buf = Vec::new();

    while let Some(chunk) = field
        .try_next()
        .await
        .map_err(|_| create_error!(source = MultipartParseError::FieldReadingError))?
    {
        buf.extend_from_slice(&chunk);
    }

    String::from_utf8(buf)
        .map_err(|_| create_error!(source = MultipartParseError::StringParsingError))
}
