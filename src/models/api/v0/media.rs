use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::domains::{Media, MediaId, MediaState};

/// API representation of a [`Media`] domain
#[derive(Debug, Serialize, Clone)]
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
    pub fn from_domain(m: Media) -> Self {
        Self {
            id: m.id,
            state: m.state,
            created_at: m.created_at,
            size: m.size,
            mimetype: m.mimetype,
            width: m.width,
            height: m.height,
            color: m.color,
        }
    }
}
