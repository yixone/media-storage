use std::slice;

use ms_shared_models::domains::{Asset, AssetId, AssetPatchData};

use crate::DbResut;

pub trait AssetRepoExt {
    async fn insert_asset(&self, asset: &Asset) -> DbResut<()>;

    async fn get_asset(&self, id: &AssetId) -> DbResut<Option<Asset>> {
        self.get_asset_by_ids(slice::from_ref(id))
            .await
            .map(|m| m.into_iter().next())
    }

    async fn get_random_asset(&self) -> DbResut<Asset>;

    async fn get_asset_by_ids(&self, ids: &[AssetId]) -> DbResut<Vec<Asset>>;

    async fn patch_asset(&self, id: &AssetId, patch: &AssetPatchData) -> DbResut<bool>;

    async fn delete_asset(&self, id: &AssetId) -> DbResut<bool>;
}
