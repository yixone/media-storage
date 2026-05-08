use ms_shared_models::domains::{Media, MediaId, MediaPatchData};

use crate::{DbResut, sqlite::SqliteDatabase, traits::MediaRepoExt};

impl MediaRepoExt for SqliteDatabase {
    async fn insert_media(&self, media: &Media) -> DbResut<()> {
        todo!()
    }

    async fn get_media_by_ids(&self, ids: &[MediaId]) -> DbResut<Vec<Media>> {
        todo!()
    }

    async fn patch_media(&self, id: &MediaId, patch: &MediaPatchData) -> DbResut<bool> {
        todo!()
    }

    async fn delete_media(&self, id: &MediaId) -> DbResut<bool> {
        todo!()
    }
}
