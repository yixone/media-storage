use ms_shared_models::domains::Media;

use crate::DbResut;

pub trait MediaRepoExt {
    async fn insert(&self, media: &Media) -> DbResut<()>;
}
