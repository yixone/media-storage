use std::{
    collections::VecDeque,
    io::Cursor,
    pin::Pin,
    task::{Context, Poll},
};

use asset_shelf_result::error::AppResult;
use bytes::Bytes;
use image::{DynamicImage, GenericImageView, ImageReader, Pixel};
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};

pub use image::{ImageFormat, imageops::FilterType};

/// Decoded image
#[derive(Debug, Clone)]
pub struct MediaImage {
    pub decoded: DynamicImage,
}

impl MediaImage {
    /// Decodes an [`MediaImage`] from AsyncRead
    pub async fn from_reader<R>(mut reader: R) -> AppResult<Self>
    where
        R: AsyncRead + Unpin + Send,
    {
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).await?;

        let reader = ImageReader::new(Cursor::new(buffer));
        let decoded = reader.with_guessed_format()?.decode()?;

        Ok(MediaImage { decoded })
    }

    /// Returns the [`MediaImage`] size in pixels
    pub fn dimension(&self) -> (u32, u32) {
        (self.decoded.width(), self.decoded.height())
    }

    /// Returns the accent color of the [`MediaImage`] in RGB
    pub fn accent_color(&self) -> [u8; 3] {
        let resized = self.decoded.resize(1, 1, FilterType::Gaussian);
        let rgb = resized.get_pixel(0, 0).to_rgb();

        rgb.0
    }

    /// Generates a thumbnail for an [`MediaImage`] with the specified width and [`FilterType`]
    pub fn thumbnail(&self, n_width: u32, filter: FilterType) -> Self {
        let (w, h) = self.dimension();
        let n_height = (h * n_width) / w;

        MediaImage {
            decoded: self.decoded.resize(n_width, n_height, filter),
        }
    }

    /// Encodes the [`MediaImage`] and returns the reader
    pub fn reader(self, format: ImageFormat) -> AppResult<MediaImageReader> {
        let mut buf = Vec::new();
        self.decoded.write_to(Cursor::new(&mut buf), format)?;
        let chunks = buf
            .chunks(16 * 1024)
            .map(Bytes::copy_from_slice)
            .collect::<VecDeque<_>>();

        Ok(MediaImageReader { inner: chunks })
    }
}

/// Encoded image reader
pub struct MediaImageReader {
    pub inner: VecDeque<Bytes>,
}

impl AsyncRead for MediaImageReader {
    fn poll_read(
        mut self: Pin<&mut Self>,
        _: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<std::io::Result<()>> {
        if buf.remaining() == 0 {
            return Poll::Ready(Ok(()));
        }

        let inner_buf = match self.inner.front_mut() {
            Some(c) => c,
            None => return Poll::Ready(Ok(())),
        };
        let len = inner_buf.len().min(buf.remaining());
        let to_put = inner_buf.split_to(len);
        buf.put_slice(&to_put);

        if inner_buf.is_empty() {
            self.inner.pop_front();
        }

        Poll::Ready(Ok(()))
    }
}
