use chrono::{DateTime, Utc};

id_type! {
    /// Media Id
    MediaId as String
}

/// Media Domain
#[derive(Debug, PartialEq)]
pub struct Media {
    /// Hash-based media Id
    pub id: MediaId,

    /// The state of media
    pub state: MediaState,

    /// Media creation time
    pub created_at: DateTime<Utc>,

    /// Media Size In Bytes
    pub size: i64,

    /// Content-Type
    pub mimetype: String,

    /// Media Width
    pub width: Option<u16>,
    /// Media Height
    pub height: Option<u16>,

    /// Media Accent Color
    pub color: Option<String>,
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
