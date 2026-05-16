use std::panic::Location;

type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;

pub type AppResult<T> = std::result::Result<T, AppError>;

/// AssetShelf Error
#[derive(Debug)]
pub struct AppError {
    /// Error Kind
    pub kind: ErrorKind,
    /// The place where the error occurred
    pub location: &'static Location<'static>,
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} occurred in {}", self.kind, self.location)
    }
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
#[cfg_attr(
    feature = "serde",
    derive(serde::Serialize),
    serde(rename_all = "SCREAMING_SNAKE_CASE", tag = "error", content = "kind")
)]
pub enum ErrorKind {
    // Asset related errors
    AssetDeleted,
    MissingAssetMedia,

    // Media related errors
    MediaTooLarge {
        size: usize,
        max: usize,
    },
    UnsupportedMediaType,
    MissingUploadMedia,
    MissingMediaBlob,

    // General errors
    NotFound,
    Internal {
        #[cfg_attr(feature = "serde", serde(skip))]
        source: BoxDynError,
    },
}

impl ErrorKind {
    pub fn is_internal(&self) -> bool {
        matches!(self, ErrorKind::Internal { .. })
    }
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
