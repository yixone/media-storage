pub(crate) type ImageResult<T> = std::result::Result<T, ImageError>;

/// Image handling errors
#[derive(Debug)]
pub enum ImageError {
    /// Image processing backend error
    BackendError(image::ImageError),
    /// IO Error
    IO(std::io::Error),
}

impl From<image::ImageError> for ImageError {
    fn from(err: image::ImageError) -> Self {
        match err {
            image::ImageError::IoError(e) => ImageError::IO(e),
            _ => ImageError::BackendError(err),
        }
    }
}

impl From<std::io::Error> for ImageError {
    fn from(err: std::io::Error) -> Self {
        ImageError::IO(err)
    }
}
