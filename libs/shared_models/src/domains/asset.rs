use chrono::{DateTime, Utc};

use crate::{domains::MediaId, id};

id_type!(AssetId as i64);

impl AssetId {
    pub fn new() -> Self {
        let id = id::generate_i64_id();
        AssetId(id)
    }
}

impl Default for AssetId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Asset {
    pub id: AssetId,
    pub media: MediaId,

    pub created_at: DateTime<Utc>,

    pub title: Option<String>,
    pub caption: Option<String>,

    pub source_url: Option<String>,
}

patch_model! {
    AssetPatchData {
        title: Option<String>,
        caption: Option<String>,
        source_url: Option<String>
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize))]
pub enum AssetError {
    AssetNotFound,
    MissingUploadMedia,
    InvalidSourceUrl,
}
