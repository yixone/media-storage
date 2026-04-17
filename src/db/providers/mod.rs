use std::path::Path;

use crate::{
    db::providers::sqlite::SqliteDb,
    error::Result,
    models::domains::{AssetsRepository, MediaRepository},
};

pub mod sqlite;

/// Database provider trait
pub trait DatabaseProvider: Send + Sync + MediaRepository + AssetsRepository {}

/// Database providers abstraction
#[derive(Debug, Clone)]
pub enum Database {
    Sqlite(SqliteDb),
}

impl Database {
    /// Opens [`SqliteDb`] from a file and returns a [`Database::Sqlite`]
    pub async fn open_sqlite(path: impl AsRef<Path>) -> Result<Self> {
        let provider = SqliteDb::open(path).await?;
        provider.migrate().await?;
        Ok(Database::Sqlite(provider))
    }
}

impl std::ops::Deref for Database {
    type Target = dyn DatabaseProvider;
    fn deref(&self) -> &Self::Target {
        match self {
            Database::Sqlite(db) => db,
        }
    }
}
