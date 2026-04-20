use std::sync::Arc;

use tokio_util::sync::CancellationToken;

use crate::bg::api::traits::WorkerUnit;

/// Host for background workers
#[derive(Clone)]
pub struct WorkersHost {
    /// Workers units list
    pub(super) units: Vec<Arc<dyn WorkerUnit>>,

    /// Workers cancellation token
    pub(super) stop: CancellationToken,
}
