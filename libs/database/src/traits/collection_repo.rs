use ms_shared_models::domains::Collection;

use crate::DbResut;

pub trait CollectionRepoExt {
    async fn insert_collection(&self, collection: &Collection) -> DbResut<()>;
}
