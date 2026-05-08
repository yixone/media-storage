use std::path::Path;

use sqlx::{
    SqlitePool,
    migrate::Migrator,
    sqlite::{SqliteAutoVacuum, SqliteConnectOptions, SqliteJournalMode, SqlitePoolOptions},
};

use crate::DbResut;

static MIGRATOR: Migrator = sqlx::migrate!("../../migrations");

#[derive(Debug, Clone)]
pub struct SqliteDatabase {
    pub(crate) pool: SqlitePool,
}

impl SqliteDatabase {
    pub async fn open(path: impl AsRef<Path>) -> DbResut<Self> {
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

    pub async fn migrate(&self) -> DbResut<()> {
        MIGRATOR.run(&self.pool).await?;
        Ok(())
    }
}
