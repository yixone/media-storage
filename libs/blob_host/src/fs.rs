use std::path::{Path, PathBuf};

use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{
    BlobHostError,
    ext::{BlobHostExt, BlobWriterExt, DynReader, DynWriter, RemoveBlobResult, RenameBlobResult},
    path::BlobPath,
};

#[derive(Debug)]
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

#[derive(Debug)]
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
