use std::str::FromStr;

use shelf_blob_host::path::BlobPath;
use uuid::Uuid;

use crate::ContentStorageError;

#[derive(Debug, Clone, PartialEq)]
pub struct StorageKey {
    pub inner: String,
}

impl StorageKey {
    pub fn from_str_unchecked(str: &str) -> Self {
        StorageKey {
            inner: str.to_string(),
        }
    }

    pub fn from_digest(d: [u8; 32]) -> Self {
        StorageKey {
            inner: hex::encode(d),
        }
    }

    pub fn to_blob_path(&self) -> BlobPath {
        BlobPath {
            inner: format!(
                "global/{}/{}/{}",
                &self.inner[..2],
                &self.inner[2..4],
                self.inner
            ),
        }
    }
}

impl AsRef<StorageKey> for StorageKey {
    fn as_ref(&self) -> &StorageKey {
        self
    }
}

impl FromStr for StorageKey {
    type Err = ContentStorageError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ContentStorageError::InvalidStorageKey);
        }

        Ok(StorageKey {
            inner: s.to_string(),
        })
    }
}

pub struct TempKey {
    pub uuid: Uuid,
}

impl TempKey {
    pub fn new() -> Self {
        TempKey {
            uuid: Uuid::new_v4(),
        }
    }

    pub fn to_blob_path(&self) -> BlobPath {
        BlobPath {
            inner: format!("temp/{}", self.uuid.simple()),
        }
    }
}

impl Default for TempKey {
    fn default() -> Self {
        Self::new()
    }
}
