use tokio::io::AsyncRead;

use crate::{BlobHostError, path::BlobPath};

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

pub(crate) type DynWriter = dyn BlobWriterExt;
pub(crate) type DynReader = dyn AsyncRead + Unpin + Send;

#[async_trait::async_trait]
pub trait BlobHostExt {
    async fn open_writer(&self, path: &BlobPath) -> Result<Box<DynWriter>, BlobHostError>;

    async fn get_reader(&self, path: &BlobPath) -> Result<Box<DynReader>, BlobHostError>;

    async fn rename(
        &self,
        from: &BlobPath,
        to: &BlobPath,
    ) -> Result<RenameBlobResult, BlobHostError>;

    async fn remove(&self, path: &BlobPath) -> Result<RemoveBlobResult, BlobHostError>;
}

#[async_trait::async_trait]
pub trait BlobWriterExt: Send + Sync {
    async fn write(&mut self, data: bytes::Bytes) -> Result<(), BlobHostError>;

    async fn finalize(self: Box<Self>) -> Result<(), BlobHostError>;
    async fn abort(self: Box<Self>) -> Result<(), BlobHostError>;
}
