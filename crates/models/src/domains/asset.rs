use chrono::{DateTime, Utc};

use crate::{
    domains::MediaId,
    ids::{id_type, tsid::TSID},
    patch::patch_model,
};

id_type! {
    /// [`TSID`] based asset identifier
    AssetId as TSID
}

impl AssetId {
    /// Generates a new ID
    pub fn new() -> Self {
        AssetId(TSID::new())
    }
}

/// Asset domain
#[derive(Debug, Clone)]
#[cfg_attr(feature = "sqlx", derive(sqlx::FromRow))]
pub struct Asset {
    /// Unique numeric ID of the Asset
    pub id: AssetId,
    /// Key for the Media associated with this Asset
    pub media_key: MediaId,

    /// Asset creation time
    pub created_at: DateTime<Utc>,

    /// Title for the asset
    pub title: Option<String>,
    /// Description for the asset
    pub caption: Option<String>,

    /// Asset source URL
    pub source_url: Option<String>,

    /// Asset deletion time
    pub deleted_at: Option<DateTime<Utc>>,
}

patch_model! {
    /// DTO for updating Asset fields
    AssetPatch {
        title: Option<String>,
        caption: Option<String>,

        source_url: Option<String>,

        deleted_at: Option<DateTime<Utc>>
    }
}

derived_error! {
    /// Asset Invariant Errors
    pub enum AssetError {
        /// Invalid source_url
        InvalidSourceUrl,

        /// Asset not found
        AssetNotFound,
        /// Attempting to modify an asset marked as deleted
        AssetDeleted,

        /// There is no media associated with the asset.
        MissingAssetMedia,
    }
}
