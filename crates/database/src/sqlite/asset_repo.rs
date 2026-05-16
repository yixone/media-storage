use asset_shelf_models::domains::{Asset, AssetId, AssetPatch};
use asset_shelf_result::error::AppResult;
use sqlx::QueryBuilder;

use crate::{SqliteDatabase, traits::AssetRepositoryExt};

impl AssetRepositoryExt for SqliteDatabase {
    async fn insert_asset(&self, asset: &Asset) -> AppResult<()> {
        sqlx::query(
            "
            INSERT INTO assets (
                id, media_key, created_at, title, caption,
                source_url, deleted_at
            )
            VALUES (
                ?, ?, ?, ?, ?,
                ?, ?
            )
            ",
        )
        .bind(&asset.id)
        .bind(&asset.media_key)
        .bind(asset.created_at)
        .bind(&asset.title)
        .bind(&asset.caption)
        .bind(&asset.source_url)
        .bind(asset.deleted_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn get_random_assets(&self, limit: u32) -> AppResult<Vec<Asset>> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let assets = sqlx::query_as(
            "
            SELECT * FROM assets
            ORDER BY RANDOM()
            LIMIT ?
            ",
        )
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(assets)
    }

    async fn get_assets(&self, limit: u32, offset: u32) -> AppResult<Vec<Asset>> {
        if limit == 0 {
            return Ok(Vec::new());
        }

        let assets = sqlx::query_as(
            "
            SELECT * FROM assets
            ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            ",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        Ok(assets)
    }

    async fn get_assets_by_ids(&self, ids: &[AssetId]) -> AppResult<Vec<Asset>> {
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

        let assets = qb.build_query_as().fetch_all(&self.pool).await?;

        Ok(assets)
    }

    async fn update_asset(&self, id: &AssetId, data: &AssetPatch) -> AppResult<bool> {
        let mut qb = QueryBuilder::new(
            "
            UPDATE assets 
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
