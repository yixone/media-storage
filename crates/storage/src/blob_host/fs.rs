use std::path::{Path, PathBuf};

use asset_shelf_result::error::AppResult;
use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::blob_host::traits::{BlobHost, BlobWriter, RemoveBlobResult, RenameBlobResult};

use super::path::BlobPath;

#[derive(Debug)]
pub struct FsBlobHost {
    mount: PathBuf,
}

impl FsBlobHost {
    pub fn new(mount: impl Into<PathBuf>) -> AppResult<Self> {
        let mount_path = mount.into();
        std::fs::create_dir_all(&mount_path)?;

        Ok(FsBlobHost { mount: mount_path })
    }

    fn make_path(&self, path: &BlobPath) -> PathBuf {
        self.mount.join(path)
    }

    async fn create_parents(&self, path: &Path) -> AppResult<()> {
        if let Some(p) = path.parent() {
            tokio::fs::create_dir_all(p).await?;
        }
        Ok(())
    }
}

impl BlobHost for FsBlobHost {
    type Writer = FsBlobWriter;
    type Reader = File;

    async fn open_writer(&self, path: &BlobPath) -> AppResult<Self::Writer> {
        let path = self.make_path(path);
        self.create_parents(&path).await?;

        let blob_file = File::create_new(&path).await?;
        let buff_writer = BufWriter::with_capacity(32 * 1024, blob_file);

        let writer = FsBlobWriter {
            writer: buff_writer,
            path,
        };

        Ok(writer)
    }

    async fn get_reader(&self, path: &BlobPath) -> AppResult<Self::Reader> {
        let path = self.make_path(path);
        let reader = File::open(path).await?;

        Ok(reader)
    }

    async fn rename(&self, from: &BlobPath, to: &BlobPath) -> AppResult<RenameBlobResult> {
        let path_from = self.make_path(from);
        let path_to = self.make_path(to);

        // TODO: Fix TOCTOU
        if tokio::fs::try_exists(&path_to).await? {
            return Ok(RenameBlobResult::AlreadyExists);
        }

        self.create_parents(&path_to).await?;
        tokio::fs::rename(&path_from, &path_to).await?;

        Ok(RenameBlobResult::Renamed)
    }

    async fn remove(&self, path: &BlobPath) -> AppResult<RemoveBlobResult> {
        let path = self.make_path(path);

        match tokio::fs::remove_file(path).await {
            Ok(_) => Ok(RemoveBlobResult::Removed),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(RemoveBlobResult::NotFound),
            Err(e) => Err(e.into()),
        }
    }
}

pub struct FsBlobWriter {
    writer: BufWriter<File>,
    path: PathBuf,
}

impl BlobWriter for FsBlobWriter {
    async fn write(&mut self, data: bytes::Bytes) -> AppResult<()> {
        self.writer.write_all(&data).await?;
        Ok(())
    }

    async fn finalize(mut self) -> AppResult<()> {
        self.writer.flush().await?;
        Ok(())
    }

    async fn abort(self) -> AppResult<()> {
        drop(self.writer);
        tokio::fs::remove_file(&self.path).await?;
        Ok(())
    }
}
