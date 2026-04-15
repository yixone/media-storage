use std::path::Path;

use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

use crate::error::Result;

/// Database migrator
static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

/// SQLite database provider
#[derive(Debug, Clone)]
pub struct SqliteDb {
    inner: SqlitePool,
}

impl SqliteDb {
    /// Opens database from a file and returns a [`SqliteDb`]
    pub async fn open(path: impl AsRef<Path>) -> Result<Self> {
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

        Ok(SqliteDb { inner: pool })
    }

    /// Applies database migrations
    pub async fn migrate(&self) -> Result<()> {
        MIGRATOR.run(&self.inner).await.map_err(sqlx::Error::from)?;
        Ok(())
    }

    /// Returns database connection pool
    pub fn pool(&self) -> &SqlitePool {
        &self.inner
    }
}

impl super::DatabaseProvider for SqliteDb {}
