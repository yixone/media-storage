use tokio::{
    fs::File,
    io::{AsyncWriteExt, BufWriter},
};

use crate::{error::Result, files::host::core::writer::FileHostWriter};

#[derive(Debug)]
pub struct FsFileHostWriter {
    pub(super) writer: BufWriter<File>,
}

#[async_trait::async_trait]
impl FileHostWriter for FsFileHostWriter {
    /// Writes the transferred data to a File Host file
    async fn write(&mut self, data: &[u8]) -> Result<()> {
        self.writer.write_all(data).await?;
        Ok(())
    }

    /// Closes the File Host file for writing and finalizes it
    async fn finalize(mut self: Box<Self>) -> Result<()> {
        self.writer.flush().await?;
        Ok(())
    }

    /// Aborts writing to the File Host file
    async fn abort(mut self: Box<Self>) -> Result<()> {
        self.writer.shutdown().await?;
        Ok(())
    }
}
