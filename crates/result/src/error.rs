use std::panic::Location;

type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type AppResult<T> = std::result::Result<T, AppError>;

/// AssetShelf Error
#[derive(Debug)]
pub struct AppError {
    /// Error Kind
    kind: ErrorKind,
    /// The place where the error occurred
    location: &'static Location<'static>,
}

impl AppError {
    /// Creates a new [`AppError`]
    #[track_caller]
    pub fn new(kind: ErrorKind) -> Self {
        AppError {
            kind,
            location: Location::caller(),
        }
    }

    /// Creates a new Internal [`AppError`] with specified source
    #[track_caller]
    pub fn internal<E>(source: E) -> Self
    where
        E: Into<BoxDynError>,
    {
        AppError::new(ErrorKind::Internal {
            source: source.into(),
        })
    }
}

/// Application error kind
#[derive(Debug)]
pub enum ErrorKind {
    // Asset related errors
    AssetDeleted,
    MissingAssetMedia,

    // Media related errors
    MediaTooLarge { size: usize, max: usize },
    UnsupportedMediaType,
    MissingUploadMedia,
    MissingMediaBlob,

    // General errors
    NotFound,
    Internal { source: BoxDynError },
}

impl<E> From<E> for AppError
where
    E: std::error::Error + Send + Sync + 'static,
{
    #[track_caller]
    fn from(err: E) -> Self {
        AppError::internal(err)
    }
}
