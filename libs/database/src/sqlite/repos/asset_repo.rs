use ms_shared_models::domains::{Asset, AssetId, AssetPatchData};
use sqlx::QueryBuilder;

use crate::{DbResut, pagination::Pagination, sqlite::SqliteDatabase, traits::AssetRepoExt};

impl AssetRepoExt for SqliteDatabase {
    async fn insert_asset(&self, asset: &Asset) -> DbResut<()> {
        sqlx::query(
            "
        INSERT INTO assets (
            id, media, created_at, title, caption, source_url
        )
        VALUES ( ?, ?, ?, ?, ?, ? )
        ",
        )
        .bind(&asset.id)
        .bind(&asset.media)
        .bind(asset.created_at)
        .bind(&asset.title)
        .bind(&asset.caption)
        .bind(&asset.source_url)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_random_assets(&self, limit: u32) -> DbResut<Vec<Asset>> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let items = sqlx::query_as(
            "
        SELECT * FROM assets
        ORDER BY RANDOM()
        LIMIT ?
        ",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    async fn get_asset_by_ids(&self, ids: &[AssetId]) -> DbResut<Vec<Asset>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let mut qb = QueryBuilder::new(
            "
        SELECT * FROM assets
        WHERE id IN
        ",
        );
        qb.push_tuples(ids, |mut qb, id| {
            qb.push_bind(id);
        });

        let list = qb.build_query_as().fetch_all(&self.pool).await?;

        Ok(list)
    }

    async fn get_assets(&self, pagination: Pagination) -> DbResut<Vec<Asset>> {
        if pagination.limit == 0 {
            return Ok(Vec::new());
        }

        let items = sqlx::query_as(
            "
        SELECT * FROM assets
        ORDER BY created_at DESC
        LIMIT ? OFFSET ?
        ",
        )
        .bind(pagination.limit)
        .bind(pagination.offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(items)
    }

    async fn patch_asset(&self, id: &AssetId, patch: &AssetPatchData) -> DbResut<bool> {
        let mut qb = QueryBuilder::new(
            "
        UPDATE assets 
        SET 
        ",
        );

        let has_changes = opt_update_query! {
            qb,
            "title" => patch.title,
            "caption" => patch.caption,
            "source_url" => patch.source_url
        };

        if !has_changes {
            return Ok(false);
        }

        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let res = qb.build().execute(&self.pool).await?;

        Ok(res.rows_affected() == 1)
    }

    async fn delete_asset(&self, id: &AssetId) -> DbResut<bool> {
        let res = sqlx::query(
            "
        DELETE FROM assets
        WHERE id = ?
        ",
        )
        .bind(id)
        .execute(&self.pool)
        .await?;

        Ok(res.rows_affected() == 1)
    }
}
