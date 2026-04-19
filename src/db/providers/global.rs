use std::{ops::Deref, path::Path};

use crate::{
    db::providers::SqliteDb,
    error::Result,
    models::domains::{AssetsRepository, MediaRepository},
};

/// Global Database provider
#[derive(Debug, Clone)]
pub enum Database {
    /// SQLite Database
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

impl Deref for Database {
    type Target = dyn DatabaseProvider;
    fn deref(&self) -> &Self::Target {
        match self {
            Database::Sqlite(db) => db,
        }
    }
}

/// Database provider supertrait
pub trait DatabaseProvider: Send + Sync + MediaRepository + AssetsRepository {}

/// Supertrait blanket implementation
impl<T: Send + Sync + MediaRepository + AssetsRepository> DatabaseProvider for T {}
