use chrono::{DateTime, Utc};

id_type! {
    /// Media identifier based on the mediafile's hash
    #[derive(Eq, Hash)]
    MediaId as String
}

id_type! {
    MediaPreviewKey as String
}

/// Media domain
#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Media {
    /// Media Id
    pub id: MediaId,
    /// Preview Key
    pub preview_key: MediaPreviewKey,

    /// Media creation time
    pub created_at: DateTime<Utc>,

    /// Media blob size
    pub blob_size: i64,
    /// Media preview blob size
    pub preview_size: i64,

    /// Media Mimetype
    pub content_type: String,

    /// Accent color for media
    pub accent_color: Option<String>,

    /// Media width in pixels
    pub width: Option<u16>,
    /// Media height in pixels
    pub height: Option<u16>,

    /// The status of media in the media life cycle
    pub status: MediaStatus,
}

patch_model! {
    /// DTO for updating Media fields
    MediaPatch {
        preview_key: MediaPreviewKey,
        preview_size: i64,

        content_type: String,

        accent_color: Option<String>,

        width: Option<u16>,
        height: Option<u16>,

        status: MediaStatus
    }
}

/// The status of media
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "sqlx", derive(sqlx::Type), sqlx(rename_all = "lowercase"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum MediaStatus {
    /// Awaiting processing
    Pending,
    /// Currently Processing
    Processing,
    /// Ready for display and distribution
    Ready,
    /// Processing ended with an error
    Failed,
}
