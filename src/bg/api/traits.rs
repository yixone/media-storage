use tokio_util::sync::CancellationToken;

use crate::error::Result;

/// Background Worker trait
#[async_trait::async_trait]
pub trait BaseWorker: Send + Sync + 'static {
    /// Starts this [`WorkerUnit`]
    async fn run(&self, cancellation: CancellationToken) -> Result<()>;
}
