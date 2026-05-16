use asset_shelf_database::SqliteDatabase;
use asset_shelf_storage::{ContentStorage, blob_host::fs::FsBlobHost};
use chrono::{DateTime, Utc};
use tokio::sync::mpsc::Sender;

use crate::bg::media_worker::MediaWorkerTask;

/// Server context for DI
pub struct ServerContext {
    pub runned_at: DateTime<Utc>,
}

/// Data Access context
pub struct DataContext {
    pub db: SqliteDatabase,
    pub store: ContentStorage<FsBlobHost>,
}

/// Context for sending messages to modules
pub struct MessagesContext {
    pub media_worker_sender: Sender<MediaWorkerTask>,
}
