use chrono::{DateTime, Utc};
use ms_shared_models::domains::{Asset, AssetId};
use serde::Serialize;

use crate::models::v1::media::ApiMedia;

#[derive(Debug, Serialize)]
pub struct ApiAsset {
    pub id: AssetId,
    pub media: ApiMedia,

    pub created_at: DateTime<Utc>,

    pub title: Option<String>,
    pub caption: Option<String>,

    pub source_url: Option<String>,

    pub is_deleted: bool,
}

impl ApiAsset {
    pub fn from_domains(a: Asset, m: impl Into<ApiMedia>) -> Self {
        ApiAsset {
            id: a.id,
            media: m.into(),
            created_at: a.created_at,
            title: a.title,
            caption: a.caption,
            source_url: a.source_url,
            is_deleted: a.is_deleted,
        }
    }
}
