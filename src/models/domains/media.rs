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
