use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::domains::{Media, MediaId, MediaState};

/// API representation of a [`Media`] domain
#[derive(Debug, Serialize)]
pub struct ApiMedia {
    pub id: MediaId,
    pub state: MediaState,
    pub created_at: DateTime<Utc>,
    pub size: i64,
    pub mimetype: String,
    pub width: Option<u16>,
    pub height: Option<u16>,
    pub color: Option<String>,
}

impl ApiMedia {
    /// Build an [`ApiMedia`] from [`Media`] domain model
    pub fn from_domain(d: Media) -> Self {
        Self {
            id: d.id,
            state: d.state,
            created_at: d.created_at,
            size: d.size,
            mimetype: d.mimetype,
            width: d.width,
            height: d.height,
            color: d.color,
        }
    }
}
