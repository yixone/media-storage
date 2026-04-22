use tokio::sync::mpsc::Sender;

use crate::{bg::media::MediaWorkerTask, db::providers::Database, files::storage::Storage};

/// Context for working with data
#[derive(Debug, Clone)]
pub struct DataContext {
    /// Database provider
    pub db: Database,
    /// Storage provider
    pub store: Storage,
}

/// Context for sending messages to background workers
#[derive(Debug, Clone)]
pub struct MsgsContext {
    /// Channel for sending tasks to the media worker
    pub media_worker_tx: Sender<MediaWorkerTask>,
}
