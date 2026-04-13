use std::sync::Arc;

use bytes::Bytes;
use futures::{Stream, TryStreamExt};
use sha2::{Digest, Sha256};

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
    /// Writes a stream to storage and returns a [`StoragePutResult`]
    pub async fn put_stream<'a, S>(&self, mut stream: S) -> Result<StoragePutResult>
    where
        S: Stream<Item = Result<Bytes>> + Unpin + 'a,
    {
        let temp = TempKey::generate().host_key();
        let mut writer = self.file_host.new_writer(&temp).await?;

        let mut size = 0;
        let mut hasher = Sha256::new();

        while let Some(chunk) = stream.try_next().await? {
            size += chunk.len();
            hasher.update(&chunk);

            if size > MAX_FILE_SIZE {
                self.file_host.delete(&temp).await?;
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
}
