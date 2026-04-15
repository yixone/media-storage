use std::{
    collections::VecDeque,
    io::Cursor,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use image::{DynamicImage, ImageFormat, ImageReader};
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};

use crate::error::Result;

/// Decoded image
#[derive(Debug)]
pub struct MediaImage {
    inner: DynamicImage,
}

impl MediaImage {
    /// Decodes an image from an [`AsyncRead`] object
    pub async fn from_reader(mut reader: impl AsyncRead + Unpin + Send) -> Result<Self> {
        // TODO: add size limit
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        let img = ImageReader::new(Cursor::new(buf))
            .with_guessed_format()?
            .decode()?;
        Ok(MediaImage { inner: img })
    }

    /// Encodes an [`MediaImage`] with the specified format and returns a [`ImageStream`]
    pub fn to_stream(self, format: ImageFormat) -> Result<ImageStream> {
        let mut buf = Vec::new();
        self.inner.write_to(Cursor::new(&mut buf), format)?;

        let chunks = buf
            .chunks(16 * 1024)
            .map(Bytes::copy_from_slice)
            .collect::<VecDeque<_>>();
        Ok(ImageStream { next: chunks })
    }
}

/// Image stream
pub struct ImageStream {
    /// Encoded image chunks
    next: VecDeque<Bytes>,
}

impl AsyncRead for ImageStream {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        let inner = match self.next.front_mut() {
            Some(c) => c,
            None => return Poll::Ready(Ok(())),
        };
        let len = inner.len().min(buf.remaining());
        let to_put = inner.split_to(len);
        buf.put_slice(&to_put);

        if inner.is_empty() {
            self.next.pop_front();
        }
        Poll::Ready(Ok(()))
    }
}
