use chrono::{DateTime, Utc};

use crate::{ids::id_type, patch::patch_model};

id_type! {
    #[derive(Eq, Hash)]
    MediaKey as String
}

/// Media domain
#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Media {
    /// Media Key. This is the key for the original file in the storage
    pub original_key: MediaKey,
    /// Preview key. This is the key for the file preview in the storage
    pub preview_key: MediaKey,

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
        preview_key: MediaKey,
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

derived_error! {
    /// Media Invariant Errors
    pub enum MediaError {
        /// The media file size is too large
        MediaTooLarge,
        /// The mimetype of the uploaded media is not supported
        UnsupportedMediaType,

        /// Media not found
        MediaNotFound,

        /// The media is in the database, but not in the storage
        MissingMediaBlob,
    }
}
