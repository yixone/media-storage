use crate::{
    db::providers::sqlite::SqliteDb,
    error::Result,
    models::domains::{Media, MediaId, MediaRepository, MediaUpdateData},
};

#[async_trait::async_trait]
impl MediaRepository for SqliteDb {
    /// Inserts a [`Media`] into the database
    async fn insert_media(&self, media: &Media) -> Result<()> {
        todo!()
    }

    /// Returns a set of [`Media`] by the list of [`MediaId`]
    async fn get_media_by_ids(&self, ids: &[&MediaId]) -> Result<Vec<Media>> {
        todo!()
    }

    /// Updates [`Media`] fields in the database
    /// according to [`MediaUpdateData`]
    async fn update_media(&self, id: &MediaId, data: &MediaUpdateData) -> Result<()> {
        todo!()
    }

    /// Deletes an [`Media`] from the database by [`MediaId`]
    async fn delete_media(&self, id: &MediaId) -> Result<()> {
        todo!()
    }
}
