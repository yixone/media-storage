use std::slice;

use asset_shelf_models::domains::{Media, MediaId, MediaPatch};
use asset_shelf_result::error::AppResult;

/// Trait for the [`Media`] repository
pub trait MediaRepositoryExt {
    /// Inserts [`Media`] into the database
    async fn insert_media(&self, media: &Media) -> AppResult<()>;

    /// Returns the [`Media`] with the specified [`MediaId`]
    async fn get_media(&self, id: &MediaId) -> AppResult<Option<Media>> {
        self.get_media_by_ids(slice::from_ref(id))
            .await
            .map(|m| m.into_iter().next())
    }

    /// Returns a list of [`Media`] by list of [`MediaId`]
    async fn get_media_by_ids(&self, ids: &[MediaId]) -> AppResult<Vec<Media>>;

    /// Updates [`Media`] fields according to the [`MediaPatch`] model
    async fn update_media(&self, id: &MediaId, data: &MediaPatch) -> AppResult<bool>;
}
