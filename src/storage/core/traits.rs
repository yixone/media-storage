#[async_trait::async_trait]
pub trait StorageIO: Send + Sync + 'static {}
