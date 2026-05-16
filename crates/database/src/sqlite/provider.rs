use std::path::Path;

use asset_shelf_result::error::AppResult;
use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

/// SQLite database provider
#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    pub(crate) pool: SqlitePool,
}

impl SqliteDatabase {
    /// Opens an SQLite database from a file and returns [`SqliteDatabase`]
    pub async fn open(path: impl AsRef<Path>) -> AppResult<Self> {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let options = SqliteConnectOptions::new()
            .filename(path)
            .create_if_missing(true)
            .auto_vacuum(SqliteAutoVacuum::Incremental)
            .journal_mode(SqliteJournalMode::Wal);

        let pool = SqlitePoolOptions::new().connect_with(options).await?;

        Ok(SqliteDatabase { pool })
    }

    /// Apply migrations to the database
    pub async fn migrate(&self) -> AppResult<()> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }
}
