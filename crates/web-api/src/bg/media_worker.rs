use std::sync::Arc;

use asset_shelf_models::domains::MediaId;
use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::di::DataContext;

/// Tasks for [`MediaWorker`]
#[derive(Debug)]
pub enum MediaWorkerTask {
    /// Complete media uploading and processing
    FinalizeUploadedMedia { id: MediaId },
}

impl MediaWorkerTask {
    /// Sends a [`MediaWorkerTask`] to the [`MediaWorker`]
    /// and returns `true` if the task was received
    pub async fn send(self, sender: &Sender<MediaWorkerTask>) -> bool {
        if let Err(e) = sender.send(self).await {
            tracing::warn!(error = ?e, "Failed to send media worker task");
            return false;
        }
        true
    }
}

/// Background Worker for media processing
pub struct MediaWorker {
    /// Worker Data Access context
    data: Arc<DataContext>,

    /// Worker tasks sender
    tx: Sender<MediaWorkerTask>,
    /// Receiver for worker tasks
    rx: Receiver<MediaWorkerTask>,
}

impl MediaWorker {
    /// Creates a new [`MediaWorker`]
    pub fn new(data: Arc<DataContext>) -> Self {
        let (tx, rx) = channel(100);
        MediaWorker { data, tx, rx }
    }

    pub fn run(self) -> Sender<MediaWorkerTask> {
        let tx = self.tx;

        tokio::spawn(async {
            // TODO: MediaWorker runtime
        });

        tx
    }
}
