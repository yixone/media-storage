use tokio_util::sync::CancellationToken;

use crate::error::Result;

/// Background Worker trait
#[async_trait::async_trait]
pub trait BaseWorker: Send + Sync + 'static {
    /// Returns this worker name
    fn worker_name(&self) -> &'static str;

    /// Starts this worker
    async fn run(&self, cancellation: CancellationToken) -> Result<()>;
}
