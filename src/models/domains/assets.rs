use chrono::{DateTime, Utc};

use crate::models::domains::MediaId;

id_type! {
    /// Asset Id
    AssetId as uuid::Uuid
}

/// Asset domain
#[derive(Debug, PartialEq)]
pub struct Asset {
    /// Asset UUID
    pub id: AssetId,

    /// Asset realted media Id
    pub media: MediaId,

    /// Asset creation time
    pub created_at: DateTime<Utc>,

    /// Asset caption
    pub caption: Option<String>,

    /// Asset source URL
    pub source_url: Option<String>,
}
