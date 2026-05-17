use asset_shelf_models::domains::{Media, MediaStatus};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct MediaApi {
    pub original_url: String,
    pub preview_url: String,

    pub created_at: DateTime<Utc>,
    pub original_size: i64,
    pub content_type: String,
    pub color: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub status: MediaStatus,
}

impl From<Media> for MediaApi {
    fn from(m: Media) -> Self {
        MediaApi {
            original_url: format!("/v1/media/{}?format=original", m.id.0),
            preview_url: format!("/v1/media/{}?format=preview", m.id.0),
            created_at: m.created_at,
            original_size: m.blob_size,
            content_type: m.content_type,
            color: m.accent_color,
            width: m.width,
            height: m.height,
            status: m.status,
        }
    }
}
