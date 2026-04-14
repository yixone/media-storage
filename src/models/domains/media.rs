use chrono::{DateTime, Utc};

id_type! {
    /// Media Id
    MediaId as String
}

/// Media Domain
#[derive(Debug, PartialEq)]
pub struct Media {
    /// Hash-based media Id
    id: MediaId,

    /// The state of media
    state: MediaState,

    /// Media creation time
    created_at: DateTime<Utc>,

    /// Media Size In Bytes
    size: i64,

    /// Content-Type
    mimetype: String,

    /// Media Width
    width: Option<u16>,
    /// Media Height
    height: Option<u16>,

    /// Media Accent Color
    color: Option<String>,
}

/// Media State
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MediaState {
    /// Media is awaiting processing
    Pending,
    /// The media is being processed.
    Processing,
    /// The media is ready to deliver
    Ready,
    /// Media processing failed
    Failed,
}
