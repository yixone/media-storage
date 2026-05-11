use shelf_blob_host::BlobHostError;

#[derive(Debug)]
pub enum ContentStorageError {
    BlobTooLarge,
    InvalidStorageKey,
    BackendError(BlobHostError),
    Io(std::io::Error),
}

impl From<BlobHostError> for ContentStorageError {
    fn from(e: BlobHostError) -> Self {
        ContentStorageError::BackendError(e)
    }
}

impl From<std::io::Error> for ContentStorageError {
    fn from(e: std::io::Error) -> Self {
        ContentStorageError::Io(e)
    }
}
