pub(crate) type MediaResult<T> = std::result::Result<T, MediaProcessingError>;

#[derive(Debug)]
pub enum MediaProcessingError {
    Image(image::ImageError),
    Io(std::io::Error),
}

impl From<std::io::Error> for MediaProcessingError {
    fn from(e: std::io::Error) -> Self {
        MediaProcessingError::Io(e)
    }
}

impl From<image::ImageError> for MediaProcessingError {
    fn from(e: image::ImageError) -> Self {
        MediaProcessingError::Image(e)
    }
}
