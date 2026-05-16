use std::slice;

use asset_shelf_models::domains::{Asset, AssetId, AssetPatch};
use asset_shelf_result::error::AppResult;

/// Trait for the [`Asset`] repository
pub trait AssetRepositoryExt {
    /// Inserts [`Asset`] into the database
    async fn insert_asset(&self, asset: &Asset) -> AppResult<()>;

    /// Returns the [`Asset`] with the specified [`AssetId`]
    async fn get_asset(&self, id: &AssetId) -> AppResult<Option<Asset>> {
        self.get_assets_by_ids(slice::from_ref(id))
            .await
            .map(|m| m.into_iter().next())
    }

    /// Returns a list of random [`Asset`]
    async fn get_random_assets(&self, limit: u32) -> AppResult<Vec<Asset>>;

    /// Returns a list of [`Asset`] with pagination
    async fn get_assets(&self, limit: u32, offset: u32) -> AppResult<Vec<Asset>>;

    /// Returns a list of [`Asset`] by list of [`AssetId`]
    async fn get_assets_by_ids(&self, ids: &[AssetId]) -> AppResult<Vec<Asset>>;

    /// Updates [`Asset`] fields according to the [`AssetPatch`] model
    async fn update_asset(&self, id: &AssetId, data: &AssetPatch) -> AppResult<bool>;
}
