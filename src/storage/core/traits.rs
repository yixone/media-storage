use tokio::io::AsyncRead;

use crate::{
    error::Result,
    storage::core::{
        types::{DeleteResult, RenameResult, StorageKey},
        writer::BlobWriter,
    },
};

#[async_trait::async_trait]
pub trait StorageIO: Send + Sync + 'static {
    /// Creates a new blob in the storage and returns the [`BlobWriter`] to it
    async fn new_writer(&self, key: &StorageKey) -> Result<Box<dyn BlobWriter>>;

    /// Returns the Reader to a file from the storage
    async fn get_reader(&self, key: &StorageKey) -> Result<Box<dyn AsyncRead + Unpin + Send>>;
}

#[async_trait::async_trait]
pub trait StorageOps: Send + Sync {
    /// Changes the key of a blob in the storage and returns the [`RenameResult`]
    async fn try_rename(&self, key: &StorageKey, dest: &StorageKey) -> Result<RenameResult>;

    /// Checks if a blob with the specified key exists in the storage
    async fn exists(&self, key: &StorageKey) -> Result<bool>;

    /// Removes a blob from storage and returns the [`DeleteResult`]
    async fn delete(&self, key: &StorageKey) -> Result<DeleteResult>;
}
