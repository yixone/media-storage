use std::sync::Arc;

use ms_content_store::ContentStorage;
use ms_database::sqlite::SqliteDatabase;
use tokio::sync::mpsc::Sender;

use crate::bg::media::MediaWorkerTask;

#[derive(Debug, Clone)]
pub struct DataContext {
    pub db: SqliteDatabase,
    pub store: Arc<ContentStorage>,
}

#[derive(Debug, Clone)]
pub struct MessagesContext {
    pub media_worker_msgs: Sender<MediaWorkerTask>,
}
