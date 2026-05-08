use std::slice;

use ms_shared_models::domains::{Media, MediaId, MediaPatchData};

use crate::DbResut;

pub trait MediaRepoExt {
    async fn insert_media(&self, media: &Media) -> DbResut<()>;

    async fn get_media(&self, id: &MediaId) -> DbResut<Option<Media>> {
        self.get_media_by_ids(slice::from_ref(id))
            .await
            .map(|m| m.into_iter().next())
    }

    async fn get_media_by_ids(&self, ids: &[MediaId]) -> DbResut<Vec<Media>>;

    // TODO: Replace bool with RepositoryResult.rows_affected
    async fn patch_media(&self, id: &MediaId, patch: &MediaPatchData) -> DbResut<bool>;

    async fn delete_media(&self, id: &MediaId) -> DbResut<bool>;
}
