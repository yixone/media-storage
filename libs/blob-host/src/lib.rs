pub mod error;
pub mod ext;
pub mod fs;
pub mod path;

use std::{ops::Deref, path::PathBuf, sync::Arc};

pub use error::BlobHostError;

use crate::{ext::BlobHostExt, fs::FsBlobHost};

pub enum BlobHost {
    Fs(Arc<FsBlobHost>),
}

impl BlobHost {
    pub fn mount_fs(mount: impl Into<PathBuf>) -> Result<Self, BlobHostError> {
        let fs_blob_host = FsBlobHost::new(mount)?;
        let blob_host = BlobHost::Fs(Arc::new(fs_blob_host));
        Ok(blob_host)
    }
}

impl Deref for BlobHost {
    type Target = dyn BlobHostExt;

    fn deref(&self) -> &Self::Target {
        match self {
            BlobHost::Fs(host) => host.as_ref(),
        }
    }
}
