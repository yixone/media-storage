use asset_shelf_database::SqliteDatabase;
use asset_shelf_storage::{ContentStorage, blob_host::fs::FsBlobHost};
use chrono::{DateTime, Utc};

/// Server context for DI
pub struct ServerContext {
    pub runned_at: DateTime<Utc>,
}

impl ServerContext {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        ServerContext {
            runned_at: Utc::now(),
        }
    }
}

/// Data Access context
pub struct DataContext {
    pub db: SqliteDatabase,
    pub store: ContentStorage<FsBlobHost>,
}

impl DataContext {
    pub fn new(db: SqliteDatabase, store: ContentStorage<FsBlobHost>) -> Self {
        DataContext { db, store }
    }
}
