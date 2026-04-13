use std::sync::Arc;

use futures::TryStreamExt;
use sha2::{Digest, Sha256};
use tokio::io::{AsyncRead, BufReader};
use tokio_util::io::ReaderStream;

use crate::{
    create_error,
    error::Result,
    files::{
        host::core::{traits::FileHost, types::RenameResult},
        storage::types::{StorageKey, StoragePutResult, TempKey},
    },
};

pub mod types;

const MAX_FILE_SIZE: usize = 128 * 1024 * 1024;

/// Content-addressable storage for files
#[derive(Clone)]
pub struct Storage {
    file_host: Arc<dyn FileHost>,
}

impl Storage {
    /// Writes a reader to storage and returns a [`StoragePutResult`]
    pub async fn put<R>(&self, mut reader: R) -> Result<StoragePutResult>
    where
        R: AsyncRead + Unpin,
    {
        let temp = TempKey::generate().host_key();
        let mut writer = self.file_host.new_writer(&temp).await?;

        let mut size = 0;
        let mut hasher = Sha256::new();

        let buf_reader = BufReader::with_capacity(32 * 1024, &mut reader);
        let mut reader = ReaderStream::new(buf_reader);

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
        let is_new = match self.file_host.try_rename(&temp, &key.host_key()).await? {
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
        let reader = self.file_host.get_reader(&key.host_key()).await?;
        Ok(reader)
    }
}
