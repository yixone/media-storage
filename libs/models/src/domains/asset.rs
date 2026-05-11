use chrono::{DateTime, Utc};

use crate::{domains::MediaKey, id_type, ids::tsid::TSID};

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
    pub media_key: MediaKey,

    /// Asset creation time
    pub created_at: DateTime<Utc>,

    /// Name for the asset
    pub name: Option<String>,
    /// Description for the asset
    pub caption: Option<String>,

    /// Asset source URL
    pub source_url: Option<String>,

    /// Is the asset marked as deleted?
    ///
    /// If `true` - the asset is displayed only on the removed assets page
    pub is_deleted: bool,
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
