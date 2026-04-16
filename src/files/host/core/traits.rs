use std::fmt::Debug;

use tokio::io::AsyncRead;

use crate::{
    error::Result,
    files::host::core::{
        types::{DeleteResult, FileHostKey, RenameResult},
        writer::FileHostWriter,
    },
};

pub trait FileHost: FileHostIO + FileHostOps + Debug {}

#[async_trait::async_trait]
pub trait FileHostIO: Send + Sync + 'static {
    /// Creates a new file in the file host and returns a [`FileHostWriter`] to it
    async fn new_writer(&self, key: &FileHostKey) -> Result<Box<dyn FileHostWriter>>;

    /// Returns the reader to a file from the file host
    async fn get_reader(&self, key: &FileHostKey) -> Result<Box<dyn AsyncRead + Unpin + Send>>;
}

#[async_trait::async_trait]
pub trait FileHostOps: Send + Sync {
    /// Changes the file key in the file host and returns the [`RenameResult`]
    async fn try_rename(&self, key: &FileHostKey, dest: &FileHostKey) -> Result<RenameResult>;

    /// checks the existence of a file with the specified key in the file host
    async fn exists(&self, key: &FileHostKey) -> Result<bool>;

    /// Removes a file from a file host and returns the [`DeleteResult`]
    async fn delete(&self, key: &FileHostKey) -> Result<DeleteResult>;
}
