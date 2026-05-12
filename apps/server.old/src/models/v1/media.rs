use chrono::{DateTime, Utc};
use serde::Serialize;
use shelf_shared_models::domains::{Media, MediaId, MediaStatus};

#[derive(Debug, Serialize)]
pub struct ApiMedia {
    pub id: MediaId,

    pub created_at: DateTime<Utc>,

    pub blob_size: i64,
    pub content_type: String,

    pub color: Option<String>,

    pub width: Option<u16>,
    pub height: Option<u16>,

    pub status: MediaStatus,
}

impl From<Media> for ApiMedia {
    fn from(m: Media) -> Self {
        ApiMedia {
            id: m.id,
            created_at: m.created_at,
            blob_size: m.blob_size,
            content_type: m.content_type,
            color: m.color,
            width: m.width,
            height: m.height,
            status: m.status,
        }
    }
}
