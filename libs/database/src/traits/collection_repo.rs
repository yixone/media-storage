use ms_shared_models::domains::Collection;

use crate::DbResut;

pub trait CollectionRepoExt {
    async fn insert(&self, collection: &Collection) -> DbResut<()>;
}
