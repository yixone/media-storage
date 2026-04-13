use std::path::{Path, PathBuf};

use tokio::{
    fs::File,
    io::{AsyncRead, BufWriter},
};

use crate::{
    error::Result,
    files::host::{
        core::{
            traits::{FileHost, FileHostIO, FileHostOps},
            types::{DeleteResult, FileHostKey, RenameResult},
            writer::FileHostWriter,
        },
        fs::writer::FsFileHostWriter,
    },
};

pub mod writer;

/// File host for local fs
#[derive(Debug)]
pub struct FsFileHost {
    root: PathBuf,
}

impl FsFileHost {
    /// Creates a new [`FsStorage`]
    pub fn new<P: Into<PathBuf>>(root_dir: P) -> Self {
        FsFileHost {
            root: root_dir.into(),
        }
    }

    /// Initialize the storage and create the necessary directories
    pub fn init(&self) -> Result<()> {
        std::fs::create_dir_all(&self.root)?;
        Ok(())
    }
}

impl FileHost for FsFileHost {}

#[async_trait::async_trait]
impl FileHostIO for FsFileHost {
    /// Creates a new file in the file host and returns a [`FileHostWriter`] to it
    async fn new_writer(&self, key: &FileHostKey) -> Result<Box<dyn FileHostWriter>> {
        let path = self.root.join(key);
        create_parents(&path).await?;
        let file = File::create_new(path).await?;
        let writer = BufWriter::with_capacity(32 * 1024, file);
        let writer = FsFileHostWriter { writer };
        Ok(Box::new(writer))
    }

    /// Returns the reader to a file from the file host
    async fn get_reader(&self, key: &FileHostKey) -> Result<Box<dyn AsyncRead + Unpin + Send>> {
        let path = self.root.join(key);
        let reader = File::open(path).await?;
        Ok(Box::new(reader))
    }
}

#[async_trait::async_trait]
impl FileHostOps for FsFileHost {
    /// Changes the file key in the file host and returns the [`RenameResult`]
    async fn try_rename(&self, key: &FileHostKey, dest: &FileHostKey) -> Result<RenameResult> {
        let path = self.root.join(key);
        let dest = self.root.join(dest);

        if tokio::fs::try_exists(&dest).await? {
            return Ok(RenameResult::AlreadyExists);
        }

        create_parents(&dest).await?;
        tokio::fs::rename(path, dest).await?;
        Ok(RenameResult::Renamed)
    }

    /// checks the existence of a file with the specified key in the file host
    async fn exists(&self, key: &FileHostKey) -> Result<bool> {
        let path = self.root.join(key);
        let exists = tokio::fs::try_exists(path).await?;
        Ok(exists)
    }

    /// Removes a file from a file host and returns the [`DeleteResult`]
    async fn delete(&self, key: &FileHostKey) -> Result<DeleteResult> {
        let path = self.root.join(key);
        match tokio::fs::remove_file(path).await {
            Ok(_) => Ok(DeleteResult::Deleted),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(DeleteResult::NotFound),
            Err(e) => Err(e.into()),
        }
    }
}

/// Creates parent directories for the file
pub async fn create_parents(path: impl AsRef<Path>) -> Result<()> {
    if let Some(parent) = path.as_ref().parent() {
        tokio::fs::create_dir_all(parent).await?;
    }
    Ok(())
}
