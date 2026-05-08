use ms_shared_models::domains::{Asset, AssetId, AssetPatchData};

use crate::{DbResut, sqlite::SqliteDatabase, traits::AssetRepoExt};

impl AssetRepoExt for SqliteDatabase {
    async fn insert_asset(&self, asset: &Asset) -> DbResut<()> {
        todo!()
    }

    async fn get_random_asset(&self) -> DbResut<Asset> {
        todo!()
    }

    async fn get_asset_by_ids(&self, ids: &[AssetId]) -> DbResut<Vec<Asset>> {
        todo!()
    }

    async fn patch_asset(&self, id: &AssetId, patch: &AssetPatchData) -> DbResut<bool> {
        todo!()
    }

    async fn delete_asset(&self, id: &AssetId) -> DbResut<bool> {
        todo!()
    }
}
