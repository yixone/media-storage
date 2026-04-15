use sqlx::QueryBuilder;

use crate::{
    db::providers::sqlite::SqliteDb,
    error::Result,
    models::domains::{Asset, AssetId, AssetUpdateData, AssetsRepository},
};

#[async_trait::async_trait]
impl AssetsRepository for SqliteDb {
    /// Inserts an [`Asset`] into the database
    async fn insert_asset(&self, asset: &Asset) -> Result<()> {
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
        .execute(self.pool())
        .await?;
        Ok(())
    }

    /// Deletes an [`Asset`] from the database by [`AssetId`]
    async fn delete_asset(&self, id: &AssetId) -> Result<()> {
        sqlx::query(
            "
        DELETE FROM assets
        WHERE id = ?
        ",
        )
        .bind(id)
        .execute(self.pool())
        .await?;
        Ok(())
    }

    /// Updates [`Asset`] fields in the database
    /// according to [`AssetUpdateData`]
    async fn update_asset(&self, id: &AssetId, data: &AssetUpdateData) -> Result<bool> {
        let mut qb = QueryBuilder::new(
            "
            UPDATE assets 
            SET 
            ",
        );
        let has_changes = opt_update_query! {
            qb,
            "title" => data.title,
            "caption" => data.caption,
            "source_url" => data.source_url
        };
        if !has_changes {
            return Ok(false);
        }

        qb.push(" WHERE id = ");
        qb.push_bind(id);

        let res = qb.build().execute(self.pool()).await?;
        Ok(res.rows_affected() == 1)
    }

    /// Returns a list of [`Asset`] with pagination
    async fn get_assets(&self, cursor: u32, limit: u32) -> Result<Vec<Asset>> {
        let items = sqlx::query_as(
            "
            SELECT * FROM assets
            LIMIT ?
            OFFSET ?
            ",
        )
        .bind(limit)
        .bind(cursor)
        .fetch_all(self.pool())
        .await?;
        Ok(items)
    }

    /// Returns a set of [`Asset`] by the list of Ids
    async fn get_assets_by_ids(&self, ids: &[&AssetId]) -> Result<Vec<Asset>> {
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

        let list = qb.build_query_as().fetch_all(self.pool()).await?;
        Ok(list)
    }
}
