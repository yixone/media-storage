use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::{
    api::v0::ApiMedia,
    domains::{Asset, AssetId},
};

/// API representation of a [`Asset`] domain
#[derive(Debug, Serialize)]
pub struct ApiAsset {
    pub id: AssetId,
    pub media: ApiMedia,
    pub created_at: DateTime<Utc>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub source_url: Option<String>,
}

impl ApiAsset {
    /// Build an [`ApiAsset`] from [`Asset`] domain and [`ApiMedia`] models
    pub fn from_domain(a: Asset, m: ApiMedia) -> Self {
        Self {
            id: a.id,
            media: m,
            created_at: a.created_at,
            title: a.title,
            caption: a.caption,
            source_url: a.source_url,
        }
    }
}
