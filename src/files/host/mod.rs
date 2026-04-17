use std::{path::PathBuf, sync::Arc};

use crate::{
    error::Result,
    files::host::{core::traits::FileHostExt, fs::FsFileHost},
};

pub mod core;
pub mod fs;

/// File hosts
#[derive(Debug, Clone)]
pub enum FileHost {
    Fs(Arc<FsFileHost>),
}

impl FileHost {
    /// Opens [`FsFileHost`] and returns a [`FileHost::Fs`]
    pub async fn fs(root_dir: impl Into<PathBuf>) -> Result<Self> {
        let host = FsFileHost::new(root_dir);
        host.init()?;
        Ok(FileHost::Fs(Arc::new(host)))
    }
}

impl std::ops::Deref for FileHost {
    type Target = dyn FileHostExt;
    fn deref(&self) -> &Self::Target {
        match self {
            FileHost::Fs(host) => host.as_ref(),
        }
    }
}
