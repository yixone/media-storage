use ms_shared_models::domains::Asset;

use crate::DbResut;

pub trait AssetRepoExt {
    async fn insert(&self, asset: &Asset) -> DbResut<()>;
}
