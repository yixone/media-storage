use asset_shelf_models::domains::{Asset, AssetId};
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::v1::MediaApi;

#[derive(Serialize)]
pub struct AssetApi {
    pub id: AssetId,
    pub media: MediaApi,
    pub created_at: DateTime<Utc>,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub source_url: Option<String>,
    pub deleted_at: Option<DateTime<Utc>>,
    pub is_deleted: bool,
}

impl AssetApi {
    /// Creates an [`AssetApi`] model from domain models
    pub fn from_domain(a: Asset, m: impl Into<MediaApi>) -> Self {
        let media = m.into();
        let is_deleted = a.deleted_at.is_some();

        AssetApi {
            id: a.id,
            media,
            created_at: a.created_at,
            title: a.title,
            caption: a.caption,
            source_url: a.source_url,
            deleted_at: a.deleted_at,
            is_deleted,
        }
    }
}
