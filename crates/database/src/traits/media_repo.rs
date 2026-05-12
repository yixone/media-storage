use std::slice;

use asset_shelf_models::domains::{Media, MediaId, MediaPatch};

use crate::DbResult;

/// Trait for the [`Media`] repository
pub trait MediaRepositoryExt {
    /// Inserts [`Media`] into the database
    async fn insert_media(&self, media: &Media) -> DbResult<()>;

    /// Returns the [`Media`] with the specified [`MediaId`]
    async fn get_media(&self, id: &MediaId) -> DbResult<Option<Media>> {
        self.get_media_by_ids(slice::from_ref(id))
            .await
            .map(|m| m.into_iter().next())
    }

    /// Returns a list of [`Media`] by list of [`MediaId`]
    async fn get_media_by_ids(&self, ids: &[MediaId]) -> DbResult<Vec<Media>>;

    /// Updates [`Media`] fields according to the [`MediaPatch`] model
    async fn update_media(&self, id: &MediaId, data: &MediaPatch) -> DbResult<bool>;
}
