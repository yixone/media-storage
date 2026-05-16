use asset_shelf_models::domains::{Media, MediaId, MediaPatch};
use asset_shelf_result::error::AppResult;
use sqlx::QueryBuilder;

use crate::{SqliteDatabase, traits::MediaRepositoryExt};

impl MediaRepositoryExt for SqliteDatabase {
    async fn insert_media(&self, media: &Media) -> AppResult<()> {
        sqlx::query(
            "
            INSERT INTO media (
                id, preview_key, created_at, blob_size, preview_size,
                content_type, accent_color, width, height, status
            )
            VALUES ( 
                ?, ?, ?, ?, ?, 
                ?, ?, ?, ?, ?
            )
            ON CONFLICT DO NOTHING
            ",
        )
        .bind(&media.id)
        .bind(&media.preview_key)
        .bind(media.created_at)
        .bind(media.blob_size)
        .bind(media.preview_size)
        .bind(&media.content_type)
        .bind(&media.accent_color)
        .bind(media.width)
        .bind(media.height)
        .bind(media.status)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_media_by_ids(&self, ids: &[MediaId]) -> AppResult<Vec<Media>> {
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

        let media = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(media)
    }

    async fn update_media(&self, id: &MediaId, data: &MediaPatch) -> AppResult<bool> {
        let mut qb = QueryBuilder::new(
            "
            UPDATE media 
            SET 
            ",
        );

        let changes = data.apply_qb(&mut qb);
        if changes == 0 {
            return Ok(false);
        }

        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let res = qb.build().execute(&self.pool).await?;

        Ok(res.rows_affected() == 1)
    }
}
