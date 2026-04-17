use sqlx::QueryBuilder;

use crate::{
    db::providers::sqlite::SqliteDb,
    error::Result,
    models::domains::{Media, MediaId, MediaRepository, MediaUpdateData},
};

#[async_trait::async_trait]
impl MediaRepository for SqliteDb {
    /// Inserts a [`Media`] into the database
    async fn insert_media(&self, media: &Media) -> Result<()> {
        sqlx::query(
            "
        INSERT INTO media (
            id, state, created_at, size, mimetype,
            width, height, color
        )
        VALUES ( 
            ?, ?, ?, ?, ?, 
            ?, ?, ?
        )
        ON CONFLICT DO NOTHING
        ",
        )
        .bind(&media.id)
        .bind(media.state)
        .bind(media.created_at)
        .bind(media.size)
        .bind(&media.mimetype)
        .bind(media.width)
        .bind(media.height)
        .bind(&media.color)
        .execute(self.pool())
        .await?;
        Ok(())
    }

    /// Deletes an [`Media`] from the database by [`MediaId`]
    async fn delete_media(&self, id: &MediaId) -> Result<()> {
        sqlx::query(
            "
        DELETE FROM media
        WHERE id = ?
        ",
        )
        .bind(id)
        .execute(self.pool())
        .await?;
        Ok(())
    }

    /// Updates [`Media`] fields in the database
    /// according to [`MediaUpdateData`]
    async fn update_media(&self, id: &MediaId, data: &MediaUpdateData) -> Result<bool> {
        let mut qb = QueryBuilder::new(
            "
        UPDATE media 
        SET 
        ",
        );
        let has_changes = opt_update_query! {
            qb,
            "state" => data.state,
            "width" => data.width,
            "height" => data.height,
            "color" => data.color
        };
        if !has_changes {
            return Ok(false);
        }

        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let res = qb.build().execute(self.pool()).await?;
        Ok(res.rows_affected() == 1)
    }

    /// Returns a set of [`Media`] by the list of [`MediaId`]
    async fn get_media_by_ids(&self, ids: &[&MediaId]) -> Result<Vec<Media>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut qb = QueryBuilder::new(
            "
        SELECT * FROM media
        WHERE id IN
        ",
        );
        qb.push_tuples(ids, |mut qb, id| {
            qb.push_bind(id);
        });

        let list = qb.build_query_as().fetch_all(self.pool()).await?;
        Ok(list)
    }
}
