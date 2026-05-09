use chrono::{DateTime, Utc};

id_type!(
    #[derive(Eq, Hash)]
    MediaId as String
);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Media {
    pub id: MediaId,

    pub created_at: DateTime<Utc>,

    pub blob_size: i64,
    pub content_type: String,

    pub color: Option<String>,

    pub width: Option<u16>,
    pub height: Option<u16>,

    pub status: MediaStatus,
}

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(rename_all = "lowercase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MediaStatus {
    Pending,
    Processing,
    Ready,
    Failed,
}

patch_model! {
    MediaPatchData {
        color: Option<String>,
        width: Option<u16>,
        height: Option<u16>,
        status: MediaStatus
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MediaError {
    InvalidMimetype,
    MediaTooLarge,
    BadMediaKey,
    MediaNotFound,
}
