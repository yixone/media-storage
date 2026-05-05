use std::{
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};

use tokio::{
    fs::File,
    io::{AsyncRead, AsyncWriteExt, BufWriter},
};

#[derive(Debug)]
pub enum BlobHostError {
    BlobTooLarge,
    InvalidBlobPath,
    BlobPathConflict,
    Io(std::io::Error),
}

impl From<std::io::Error> for BlobHostError {
    fn from(e: std::io::Error) -> Self {
        BlobHostError::Io(e)
    }
}

#[derive(Debug, Clone)]
pub struct BlobPath {
    pub inner: String,
}

impl BlobPath {
    pub fn new(path: impl Into<String>) -> Self {
        BlobPath { inner: path.into() }
    }
}

impl AsRef<Path> for BlobPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.inner)
    }
}

pub enum RenameBlobResult {
    Renamed,
    AlreadyExists,
}

pub enum RemoveBlobResult {
    Removed,
    NotFound,
}

type DynWriter = dyn BlobWriterExt;
type DynReader = dyn AsyncRead + Unpin + Send;

#[async_trait::async_trait]
pub trait BlobHostExt {
    async fn open_writer(&self, path: &BlobPath) -> Result<Box<DynWriter>, BlobHostError>;

    async fn get_reader(&self, path: &BlobPath) -> Result<Box<DynReader>, BlobHostError>;

    async fn rename(
        &self,
        from: &BlobPath,
        to: &BlobPath,
    ) -> Result<RenameBlobResult, BlobHostError>;

    async fn remove(&self, path: &BlobPath) -> Result<RemoveBlobResult, BlobHostError>;
}

#[async_trait::async_trait]
pub trait BlobWriterExt {
    async fn write(&mut self, data: bytes::Bytes) -> Result<(), BlobHostError>;

    async fn finalize(self: Box<Self>) -> Result<(), BlobHostError>;
    async fn abort(self: Box<Self>) -> Result<(), BlobHostError>;
}

pub struct FsBlobHost {
    mount: PathBuf,
}

impl FsBlobHost {
    pub fn new(mount: impl Into<PathBuf>) -> Result<Self, BlobHostError> {
        let mount_path = mount.into();
        std::fs::create_dir_all(&mount_path)?;

        Ok(FsBlobHost { mount: mount_path })
    }

    fn assemble_path(&self, p: &BlobPath) -> PathBuf {
        self.mount.join(p)
    }

    async fn create_parents(&self, path: &Path) -> Result<(), BlobHostError> {
        if let Some(p) = path.parent() {
            tokio::fs::create_dir_all(p).await?;
        }
        Ok(())
    }
}

#[async_trait::async_trait]
impl BlobHostExt for FsBlobHost {
    async fn open_writer(&self, path: &BlobPath) -> Result<Box<DynWriter>, BlobHostError> {
        let fs_path = self.assemble_path(path);
        self.create_parents(&fs_path).await?;

        let blob_file = File::options()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&fs_path)
            .await?;
        let buf_writer = BufWriter::with_capacity(32 * 1024, blob_file);

        let writer = FsBlobWriter {
            writer: buf_writer,
            target: fs_path,
        };
        Ok(Box::new(writer))
    }

    async fn get_reader(&self, path: &BlobPath) -> Result<Box<DynReader>, BlobHostError> {
        let fs_path = self.assemble_path(path);
        let reader = File::options().read(true).open(fs_path).await?;
        Ok(Box::new(reader))
    }

    async fn rename(
        &self,
        from: &BlobPath,
        to: &BlobPath,
    ) -> Result<RenameBlobResult, BlobHostError> {
        let fs_from_path = self.assemble_path(from);
        let fs_to_path = self.assemble_path(to);

        // TODO: Fix TOCTOU
        if tokio::fs::try_exists(&fs_to_path).await? {
            return Ok(RenameBlobResult::AlreadyExists);
        }

        self.create_parents(&fs_to_path).await?;
        tokio::fs::rename(&fs_from_path, &fs_to_path).await?;
        Ok(RenameBlobResult::Renamed)
    }

    async fn remove(&self, path: &BlobPath) -> Result<RemoveBlobResult, BlobHostError> {
        let fs_path = self.assemble_path(path);
        match tokio::fs::remove_file(fs_path).await {
            Ok(_) => Ok(RemoveBlobResult::Removed),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(RemoveBlobResult::NotFound),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct FsBlobWriter {
    pub writer: BufWriter<File>,
    pub target: PathBuf,
}

#[async_trait::async_trait]
impl BlobWriterExt for FsBlobWriter {
    async fn write(&mut self, data: bytes::Bytes) -> Result<(), BlobHostError> {
        self.writer.write_all(&data).await?;
        Ok(())
    }

    async fn finalize(mut self: Box<Self>) -> Result<(), BlobHostError> {
        self.writer.flush().await?;
        Ok(())
    }

    async fn abort(mut self: Box<Self>) -> Result<(), BlobHostError> {
        self.writer.shutdown().await?;
        // TODO: A more optimal way to clean after abort?
        tokio::fs::remove_file(&self.target).await?;
        Ok(())
    }
}

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
