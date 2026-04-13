use crate::error::Result;

#[async_trait::async_trait]
pub trait FileHostWriter: Send + Sync {
    /// Writes the transferred data to a File Host file
    async fn write(&mut self, data: &[u8]) -> Result<()>;

    /// Closes the File Host file for writing and finalizes it
    async fn finalize(self: Box<Self>) -> Result<()>;

    /// Aborts writing to the File Host file
    async fn abort(self: Box<Self>) -> Result<()>;
}
