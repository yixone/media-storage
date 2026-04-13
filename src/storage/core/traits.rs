use tokio::io::AsyncRead;

use crate::{
    error::Result,
    storage::core::{
        types::{DeleteResult, RenameResult, StorageKey},
        writer::StorageWriter,
    },
};

pub trait FileHost: StorageIO + StorageOps {}

#[async_trait::async_trait]
pub trait StorageIO: Send + Sync + 'static {
    /// Creates a new file in the storage and returns the [`StorageWriter`] to it
    async fn new_writer(&self, key: &StorageKey) -> Result<Box<dyn StorageWriter>>;

    /// Returns the Reader to a file from the storage
    async fn get_reader(&self, key: &StorageKey) -> Result<Box<dyn AsyncRead + Unpin + Send>>;
}

#[async_trait::async_trait]
pub trait StorageOps: Send + Sync {
    /// Changes the key of a file in the storage and returns the [`RenameResult`]
    async fn try_rename(&self, key: &StorageKey, dest: &StorageKey) -> Result<RenameResult>;

    /// Checks if a file with the specified key exists in the storage
    async fn exists(&self, key: &StorageKey) -> Result<bool>;

    /// Removes a file from storage and returns the [`DeleteResult`]
    async fn delete(&self, key: &StorageKey) -> Result<DeleteResult>;
}
