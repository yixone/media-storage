use crate::error::Result;

#[async_trait::async_trait]
pub trait BlobWriter: Send + Sync {
    /// Writes the transferred data to a blob
    async fn write(&mut self, data: &[u8]) -> Result<()>;

    /// Closes the blob for writing and finalizes it
    async fn finalize(self: Box<Self>) -> Result<()>;

    /// Aborts writing to the blob
    async fn abort(self: Box<Self>) -> Result<()>;
}
