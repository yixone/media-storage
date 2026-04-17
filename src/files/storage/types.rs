use uuid::Uuid;

use crate::files::host::core::types::FileHostKey;

/// Key for the file in the storage
#[derive(Debug, Clone, PartialEq)]
pub struct StorageKey {
    pub inner: String,
}

impl StorageKey {
    /// Creates a [`StorageKey`] from a 32-bit buf
    pub fn from_digest(d: [u8; 32]) -> Self {
        StorageKey {
            inner: hex::encode(d),
        }
    }

    /// Creates [`StorageKey`] from a [`FileHostKey`]
    pub fn to_host_key(&self) -> FileHostKey {
        FileHostKey {
            inner: format!(
                "global/{}/{}/{}",
                &self.inner[..2],
                &self.inner[2..4],
                self.inner
            ),
        }
    }
}

/// Temporary key in Storage
#[derive(Debug, Clone, PartialEq)]
pub struct TempKey {
    inner: Uuid,
}

impl TempKey {
    /// Generates a new [`TempKey`]
    pub fn generate() -> Self {
        TempKey {
            inner: Uuid::new_v4(),
        }
    }

    /// Creates [`TempKey`] from a [`FileHostKey`]
    pub fn to_host_key(&self) -> FileHostKey {
        FileHostKey {
            inner: format!("temp/{}", self.inner.as_simple()),
        }
    }
}

/// Result of writing data to storage
#[derive(Debug)]
pub struct StoragePutResult {
    pub key: StorageKey,
    pub size: usize,
    pub is_new: bool,
}
