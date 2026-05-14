use std::str::FromStr;

use crate::{StorageError, blob_host::path::BlobPath};

/// Hash address of the blob in storage
#[derive(Debug, Clone, PartialEq)]
pub struct StorageKey {
    inner: String,
}

impl StorageKey {
    /// Returns [`StorageKey`] from [`str`] without checking
    pub fn from_str_unchecked(str: &str) -> Self {
        StorageKey {
            inner: str.to_string(),
        }
    }

    /// Returns [`StorageKey`] from a 32-byte slice
    pub fn from_digest(digest: &[u8; 32]) -> Self {
        StorageKey {
            inner: hex::encode(digest),
        }
    }

    /// Returns [`BlobPath`] in storage
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

    /// Returns the [`StorageKey`] inner as [`str`]
    pub fn as_str(&self) -> &str {
        &self.inner
    }
}

impl AsRef<StorageKey> for StorageKey {
    fn as_ref(&self) -> &StorageKey {
        self
    }
}

impl FromStr for StorageKey {
    type Err = StorageError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 64 || !s.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(StorageError::InvalidKey);
        }

        Ok(StorageKey {
            inner: s.to_string(),
        })
    }
}
