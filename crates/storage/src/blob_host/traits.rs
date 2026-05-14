use tokio::io::AsyncRead;

use crate::{StorageResult, blob_host::path::BlobPath};

#[derive(Debug, PartialEq)]
pub enum RenameBlobResult {
    Renamed,
    AlreadyExists,
}

#[derive(Debug, PartialEq)]
pub enum RemoveBlobResult {
    Removed,
    NotFound,
}

/// Blob host for storing and working with blobs
pub trait BlobHost {
    type Writer: BlobWriter;
    type Reader: AsyncRead + Unpin + Send;

    /// Creates a new blob and returns a [`BlobWriter`] for writing to it
    async fn open_writer(&self, path: &BlobPath) -> StorageResult<Self::Writer>;

    /// Returns a reader for the blob at the specified path
    async fn get_reader(&self, path: &BlobPath) -> StorageResult<Self::Reader>;

    /// Changes the path of a blob
    async fn rename(&self, from: &BlobPath, to: &BlobPath) -> StorageResult<RenameBlobResult>;

    /// Removes a blob from the host
    async fn remove(&self, path: &BlobPath) -> StorageResult<RemoveBlobResult>;
}

/// Blob writer for chunked upload of blobs to the host
pub trait BlobWriter: Send + Sync {
    /// Writes a data to the current blob
    async fn write(&mut self, data: bytes::Bytes) -> StorageResult<()>;

    /// Finalizes the current blob and closes the writing
    async fn finalize(self) -> StorageResult<()>;

    /// Aborts writing to the current blob and deletes it
    async fn abort(self) -> StorageResult<()>;
}
