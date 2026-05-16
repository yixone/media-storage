use asset_shelf_models::domains::{Media, MediaId, MediaStatus};
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize)]
pub struct MediaApi {
    pub id: MediaId,
    pub created_at: DateTime<Utc>,
    pub size: i64,
    pub content_type: String,
    pub color: Option<String>,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub status: MediaStatus,
}

impl From<Media> for MediaApi {
    fn from(m: Media) -> Self {
        MediaApi {
            id: m.id,
            created_at: m.created_at,
            size: m.blob_size,
            content_type: m.content_type,
            color: m.accent_color,
            width: m.width,
            height: m.height,
            status: m.status,
        }
    }
}
