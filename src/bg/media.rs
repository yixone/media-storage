use tokio::sync::mpsc::{Receiver, Sender, channel};

use crate::{
    db::providers::Database, error::Result, files::storage::Storage, models::domains::MediaId,
};

/// Background Media worker
pub struct MediaWorker {
    service: WorkerService,
    queue: WorkerQueue,
}

impl MediaWorker {
    /// Creates a new [`MediaWorker`]
    pub fn new(db: Database, store: Storage) -> Self {
        let (tx, rx) = channel(128);
        let queue = WorkerQueue { rx, tx };

        let service = WorkerService { db, store };

        Self { service, queue }
    }

    /// Returns a [`Sender`] for sending [`MediaWorkerTask`] to the worker queue
    pub fn sender(&self) -> Sender<MediaWorkerTask> {
        self.queue.tx.clone()
    }

    /// The main loop of a worker waiting for signals to perform a task
    pub async fn run(self) -> Result<()> {
        tokio::spawn(async move {
            let this = self;
        });
        Ok(())
    }
}

/// [`MediaWorker`] Tasks
pub enum MediaWorkerTask {
    /// New media has been uploaded and needs to be processed
    NewMedia { id: MediaId },
}

/// [`MediaWorker`] queue container
struct WorkerQueue {
    rx: Receiver<MediaWorkerTask>,
    tx: Sender<MediaWorkerTask>,
}

/// [`MediaWorker`] service
struct WorkerService {
    db: Database,
    store: Storage,
}
