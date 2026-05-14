use futures::TryStreamExt;
use sha2::{Digest, Sha256};
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;
use uuid::Uuid;

use crate::{
    StorageError, StorageKey, StorageResult,
    blob_host::{
        path::BlobPath,
        traits::{BlobHost, BlobWriter, RenameBlobResult},
    },
};

/// Result of putting to [`ContentStorage`]
pub struct StoragePutResult {
    /// Blob key
    pub key: StorageKey,
    /// Blob size in bytes
    pub size: usize,
    /// There was no blob before
    pub is_new: bool,
}

/// Content-addressable storage
pub struct ContentStorage<B: BlobHost> {
    /// Storage backend for blobs
    pub backend: B,
    /// Max storage blob size
    pub max_size: usize,
}

impl<B: BlobHost> ContentStorage<B> {
    /// Creates a new [`ContentStorage`]
    pub fn new(backend: B, max_size: usize) -> Self {
        ContentStorage { backend, max_size }
    }

    /// Puts a blob into storage and returns [`StoragePutResult`]
    pub async fn put<R>(&self, data: R) -> StorageResult<StoragePutResult>
    where
        R: AsyncRead + Unpin,
    {
        let temp_id = Uuid::new_v4().simple().to_string();
        let temp_path = BlobPath::new(format!("temp/{temp_id}"));

        let mut size = 0;
        let mut hash = Sha256::new();

        let mut data_reader = ReaderStream::with_capacity(data, 32 * 1024);
        let mut blob_writer = self.backend.open_writer(&temp_path).await?;

        while let Some(chunk) = data_reader.try_next().await? {
            size += chunk.len();

            if size > self.max_size {
                blob_writer.abort().await?;
                return Err(StorageError::BlobTooLarge {
                    size,
                    max: self.max_size,
                });
            }

            hash.update(&chunk);
            blob_writer.write(chunk).await?;
        }
        blob_writer.finalize().await?;

        let key_hash = hash.finalize().into();
        let key = StorageKey::from_digest(&key_hash);
        let path = key.to_blob_path();

        let is_new = match self.backend.rename(&temp_path, &path).await? {
            RenameBlobResult::Renamed => true,
            RenameBlobResult::AlreadyExists => {
                self.backend.remove(&temp_path).await?;
                false
            }
        };
        let put_result = StoragePutResult { key, size, is_new };

        Ok(put_result)
    }

    /// Returns a reader for a blob from [`ContentStorage`] by [`StorageKey`]
    pub async fn get<K>(&self, key: K) -> StorageResult<B::Reader>
    where
        K: AsRef<StorageKey>,
    {
        let key_ref = key.as_ref();
        let path = key_ref.to_blob_path();

        let reader = self.backend.get_reader(&path).await?;

        Ok(reader)
    }
}
