pub(crate) type MediaResult<T> = std::result::Result<T, MediaError>;

pub enum MediaError {
    Image(image::ImageError),
    Io(std::io::Error),
}

impl From<std::io::Error> for MediaError {
    fn from(e: std::io::Error) -> Self {
        MediaError::Io(e)
    }
}

impl From<image::ImageError> for MediaError {
    fn from(e: image::ImageError) -> Self {
        MediaError::Image(e)
    }
}
