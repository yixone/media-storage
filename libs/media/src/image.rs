use std::{
    collections::VecDeque,
    io::Cursor,
    pin::Pin,
    task::{Context, Poll},
};

use bytes::Bytes;
use image::{
    DynamicImage, GenericImageView, ImageFormat, ImageReader, Pixel, imageops::FilterType,
};
use tokio::io::{AsyncRead, AsyncReadExt, ReadBuf};

use crate::error::MediaResult;

pub struct MediaImage {
    inner: DynamicImage,
}

impl MediaImage {
    pub async fn from_reader(mut reader: impl AsyncRead + Unpin + Send) -> MediaResult<Self> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf).await?;
        let img = ImageReader::new(Cursor::new(buf))
            .with_guessed_format()?
            .decode()?;
        Ok(MediaImage { inner: img })
    }

    pub fn get_color(&self) -> [u8; 3] {
        let comp = self.inner.resize(1, 1, FilterType::Gaussian);
        let rgb = comp.get_pixel(0, 0).to_rgb();
        rgb.0
    }

    pub fn get_dimension(&self) -> (u32, u32) {
        (self.inner.width(), self.inner.height())
    }

    pub fn to_stream(self, format: ImageFormat) -> MediaResult<ImageStream> {
        let mut buf = Vec::new();
        self.inner.write_to(Cursor::new(&mut buf), format)?;
        let chunks = buf
            .chunks(16 * 1024)
            .map(Bytes::copy_from_slice)
            .collect::<VecDeque<_>>();
        Ok(ImageStream { next: chunks })
    }
}

pub struct ImageStream {
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

        let inner_buf = match self.next.front_mut() {
            Some(c) => c,
            None => return Poll::Ready(Ok(())),
        };
        let len = inner_buf.len().min(buf.remaining());
        let to_put = inner_buf.split_to(len);
        buf.put_slice(&to_put);

        if inner_buf.is_empty() {
            self.next.pop_front();
        }

        Poll::Ready(Ok(()))
    }
}
