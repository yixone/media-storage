pub mod error;
pub mod key;

use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use sha2::{Digest, Sha256};
use shelf_blob_host::{BlobHost, ext::RenameBlobResult};
use tokio::io::AsyncRead;
use tokio_util::io::{ReaderStream, StreamReader};

pub use error::ContentStorageError;

use crate::key::{StorageKey, TempKey};

#[derive(Debug)]
pub struct StoragePutResult {
    pub key: StorageKey,
    pub size: usize,
    pub is_new: bool,
}

#[derive(Debug)]
pub struct ContentStorage {
    pub backend: BlobHost,
    pub max_size: usize,
}

impl ContentStorage {
    pub fn new(backend: BlobHost, max_size: usize) -> Self {
        ContentStorage { backend, max_size }
    }

    pub async fn put_stream<S>(&self, stream: S) -> Result<StoragePutResult, ContentStorageError>
    where
        S: Stream<Item = Result<Bytes, std::io::Error>> + Unpin,
    {
        self.put(StreamReader::new(stream)).await
    }

    pub async fn put<R>(&self, reader: R) -> Result<StoragePutResult, ContentStorageError>
    where
        R: AsyncRead + Unpin,
    {
        let temp_key = TempKey::new();
        let temp_path = temp_key.to_blob_path();

        let mut size = 0;
        let mut hasher = Sha256::new();

        let mut input_reader = ReaderStream::with_capacity(reader, 32 * 1024);
        let mut writer = self.backend.open_writer(&temp_path).await?;

        while let Some(chunk) = input_reader.try_next().await? {
            size += chunk.len();
            hasher.update(&chunk);

            if size > self.max_size {
                writer.abort().await?;
                return Err(ContentStorageError::BlobTooLarge);
            }

            writer.write(chunk).await?;
        }
        writer.finalize().await?;

        let key = StorageKey::from_digest(hasher.finalize().into());
        let is_new = match self.backend.rename(&temp_path, &key.to_blob_path()).await? {
            RenameBlobResult::Renamed => true,
            RenameBlobResult::AlreadyExists => {
                self.backend.remove(&temp_path).await?;
                false
            }
        };
        let result = StoragePutResult { key, size, is_new };
        Ok(result)
    }

    pub async fn get(
        &self,
        key: impl AsRef<StorageKey>,
    ) -> Result<impl AsyncRead + Unpin + Send + 'static, ContentStorageError> {
        let path = key.as_ref().to_blob_path();
        let reader = self.backend.get_reader(&path).await?;
        Ok(reader)
    }
}
