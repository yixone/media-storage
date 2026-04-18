use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use sha2::{Digest, Sha256};
use tokio::io::AsyncRead;
use tokio_util::io::{ReaderStream, StreamReader};

use crate::{
    create_error,
    error::Result,
    files::{
        host::{FileHost, core::types::RenameResult},
        storage::types::{StorageKey, StoragePutResult, TempKey},
    },
};

pub mod types;

const MAX_FILE_SIZE: usize = 128 * 1024 * 1024;

/// Content-addressable storage for files
#[derive(Debug, Clone)]
pub struct Storage {
    file_host: FileHost,
}

impl Storage {
    /// Creates a new [`Storage`]
    pub fn new(file_host: FileHost) -> Self {
        Self { file_host }
    }

    /// Writes a Stream to storage and returns a [`StoragePutResult`]
    pub async fn put_stream<S>(&self, stream: S) -> Result<StoragePutResult>
    where
        S: Stream<Item = Result<Bytes>> + Unpin,
    {
        // FIXME:
        // We need to decide how to handle the error correctly,
        // because we're currently mapping:
        // Original -> AppError -> std::io::Error

        self.put(StreamReader::new(stream.map_err(std::io::Error::other)))
            .await
    }

    /// Writes a reader to storage and returns a [`StoragePutResult`]
    pub async fn put<R>(&self, mut reader: R) -> Result<StoragePutResult>
    where
        R: AsyncRead + Unpin,
    {
        let temp = TempKey::generate().to_host_key();
        let mut writer = self.file_host.new_writer(&temp).await?;

        let mut size = 0;
        let mut hasher = Sha256::new();

        let mut reader = ReaderStream::with_capacity(&mut reader, 32 * 1024);

        while let Some(chunk) = reader.try_next().await? {
            size += chunk.len();
            hasher.update(&chunk);

            if size > MAX_FILE_SIZE {
                writer.abort().await?;
                return Err(create_error!(FileTooLarge {
                    max_size: MAX_FILE_SIZE,
                    received: size
                }));
            }

            writer.write(&chunk).await?;
        }
        writer.finalize().await?;

        let key = StorageKey::from_digest(hasher.finalize().into());
        let is_new = match self.file_host.try_rename(&temp, &key.to_host_key()).await? {
            RenameResult::Renamed => true,
            RenameResult::AlreadyExists => {
                self.file_host.delete(&temp).await?;
                false
            }
        };

        Ok(StoragePutResult { key, size, is_new })
    }

    /// Returns the Reader to a file from the storage
    pub async fn get(&self, key: &StorageKey) -> Result<impl AsyncRead + Send + Unpin> {
        let reader = self.file_host.get_reader(&key.to_host_key()).await?;
        Ok(reader)
    }
}
