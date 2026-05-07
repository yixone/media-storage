use chrono::{DateTime, Utc};

id_type!(MediaId as String);

#[derive(Debug, Clone)]
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

#[derive(Debug, Clone, Copy)]
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
