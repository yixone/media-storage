use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::bg::api::traits::BaseWorker;

/// Host for background workers
#[derive(Clone)]
pub struct WorkersHost {
    /// Workers units list
    pub(super) units: Vec<Arc<dyn BaseWorker>>,

    /// Workers cancellation token
    pub(super) cancellation: CancellationToken,
}

impl WorkersHost {
    /// Creates a new [`WorkersHost`]
    pub fn new() -> Self {
        let cancellation = CancellationToken::new();
        WorkersHost {
            units: Vec::new(),
            cancellation,
        }
    }

    /// Creates a new [`WorkersHost`] with [`CancellationToken`]
    pub fn with_cancellation(cancellation: CancellationToken) -> Self {
        WorkersHost {
            units: Vec::new(),
            cancellation,
        }
    }

    /// Adds the given [`BaseWorker`] to the [`WorkersHost`]
    pub fn with_worker(mut self, unit: impl BaseWorker) -> Self {
        self.units.push(Arc::new(unit));
        self
    }

    /// Starts [`WorkersHost`] units
    pub async fn run(&self) {
        if self.units.is_empty() {
            return;
        }
        for u in &self.units {
            let cancellation = self.cancellation.clone();
            let unit = u.clone();

            tokio::spawn(async move {
                tracing::info!(worker = unit.worker_name(), "worker_host.running_worker");

                let rt = unit.run(cancellation).await;
                if let Err(e) = rt {
                    tracing::error!(error = ?e, worker = unit.worker_name(), "worker_host.runtime_error");
                }
            });
        }
    }

    /// Causes all background workers to stop
    pub async fn stop(self) {
        self.cancellation.cancel();
    }
}

impl Default for WorkersHost {
    fn default() -> Self {
        Self::new()
    }
}
