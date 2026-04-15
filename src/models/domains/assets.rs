use chrono::{DateTime, Utc};

use crate::{
    error::Result,
    models::{domains::MediaId, types::UpdateField},
};

id_type! {
    /// Asset Id
    AssetId as uuid::Uuid
}

/// Asset domain
#[derive(Debug, PartialEq, sqlx::FromRow)]
pub struct Asset {
    /// Asset UUID
    pub id: AssetId,

    /// Asset realted media Id
    pub media: MediaId,

    /// Asset creation time
    pub created_at: DateTime<Utc>,

    /// Asset title
    pub title: Option<String>,

    /// Asset caption
    pub caption: Option<String>,

    /// Asset source URL
    pub source_url: Option<String>,
}

/// Asset Update DTO
#[derive(Debug, Default)]
pub struct AssetUpdateData {
    /// New Asset title
    pub title: UpdateField<Option<String>>,
    /// New Asset caption
    pub caption: UpdateField<Option<String>>,

    /// New Asset source URL
    pub source_url: UpdateField<Option<String>>,
}

/// Trait for [`Asset`] domain repository
#[async_trait::async_trait]
pub trait AssetsRepository {
    /// Inserts an [`Asset`] into the database
    async fn insert_asset(&self, asset: &Asset) -> Result<()>;

    /// Deletes an [`Asset`] from the database by [`AssetId`]
    async fn delete_asset(&self, id: &AssetId) -> Result<()>;

    /// Updates [`Asset`] fields in the database
    /// according to [`AssetUpdateData`]
    async fn update_asset(&self, id: &AssetId, data: &AssetUpdateData) -> Result<bool>;

    /// Returns a list of [`Asset`] with pagination
    async fn get_assets(&self, cursor: u32, limit: u32) -> Result<Vec<Asset>>;

    /// Returns an [`Asset`] from the database by [`AssetId`]
    async fn get_asset(&self, id: &AssetId) -> Result<Option<Asset>> {
        self.get_assets_by_ids(&[id])
            .await
            .map(|a| a.into_iter().next())
    }

    /// Returns a set of [`Asset`] by the list of Ids
    async fn get_assets_by_ids(&self, ids: &[&AssetId]) -> Result<Vec<Asset>>;
}
