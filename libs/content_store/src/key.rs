use ms_blob_host::path::BlobPath;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct StorageKey {
    pub inner: String,
}

impl StorageKey {
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
