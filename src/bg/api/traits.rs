use tokio_util::sync::CancellationToken;

use crate::error::Result;

/// Background Worker Unit trait
#[async_trait::async_trait]
pub trait WorkerUnit: Send + Sync + 'static {
    /// Starts a this [`WorkerUnit`] with the passed [`CancellationToken`]
    async fn run(&self, cancellation: CancellationToken) -> Result<()>;
}
