use std::slice;

use shelf_shared_models::domains::{Asset, AssetId, AssetPatchData};

use crate::{DbResut, pagination::Pagination};

pub trait AssetRepoExt {
    async fn insert_asset(&self, asset: &Asset) -> DbResut<()>;

    async fn get_asset(&self, id: &AssetId) -> DbResut<Option<Asset>> {
        self.get_asset_by_ids(slice::from_ref(id))
            .await
            .map(|m| m.into_iter().next())
    }

    async fn get_random_assets(&self, limit: u32) -> DbResut<Vec<Asset>>;

    async fn get_asset_by_ids(&self, ids: &[AssetId]) -> DbResut<Vec<Asset>>;

    async fn get_assets(&self, pagination: Pagination) -> DbResut<Vec<Asset>>;

    async fn patch_asset(&self, id: &AssetId, patch: &AssetPatchData) -> DbResut<bool>;

    async fn delete_asset(&self, id: &AssetId) -> DbResut<bool>;
}
