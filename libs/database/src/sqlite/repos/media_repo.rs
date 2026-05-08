use ms_shared_models::domains::{Media, MediaId, MediaPatchData};
use sqlx::QueryBuilder;

use crate::{DbResut, sqlite::SqliteDatabase, traits::MediaRepoExt};

impl MediaRepoExt for SqliteDatabase {
    async fn insert_media(&self, media: &Media) -> DbResut<()> {
        sqlx::query(
            "
        INSERT INTO media (
            id, status, created_at, blob_size, content_type,
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
        .bind(media.status)
        .bind(media.created_at)
        .bind(media.blob_size)
        .bind(&media.content_type)
        .bind(media.width)
        .bind(media.height)
        .bind(&media.color)
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn get_media_by_ids(&self, ids: &[MediaId]) -> DbResut<Vec<Media>> {
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

        let list = qb.build_query_as().fetch_all(&self.pool).await?;
        Ok(list)
    }

    async fn patch_media(&self, id: &MediaId, patch: &MediaPatchData) -> DbResut<bool> {
        let mut qb = QueryBuilder::new(
            "
        UPDATE media 
        SET 
        ",
        );

        let has_changes = opt_update_query! {
            qb,
            "status" => patch.status,
            "width" => patch.width,
            "height" => patch.height,
            "color" => patch.color
        };

        if !has_changes {
            return Ok(false);
        }

        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let res = qb.build().execute(&self.pool).await?;

        Ok(res.rows_affected() == 1)
    }

    async fn delete_media(&self, id: &MediaId) -> DbResut<bool> {
        let res = sqlx::query(
            "
        DELETE FROM media
        WHERE id = ?
        ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(res.rows_affected() == 1)
    }
}
