use actix_multipart::Field;
use futures::TryStreamExt;

use crate::error::{AppError, AppResult};

pub async fn read_field_to_string(field: &mut Field) -> AppResult<String> {
    let mut buf = Vec::new();
    while let Some(chunk) = field.try_next().await? {
        buf.extend_from_slice(&chunk);
    }
    String::from_utf8(buf).map_err(|_| AppError::MultipartError)
}
