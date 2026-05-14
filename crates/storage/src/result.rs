pub(crate) type StorageResult<T> = std::result::Result<T, StorageError>;

/// Storage module errors
#[derive(Debug)]
pub enum StorageError {
    /// The blob size is too large
    BlobTooLarge { size: usize, max: usize },

    /// The repository received a key
    /// that is not the hash address of the file
    InvalidKey,

    /// The storage does not contain a blob
    NotFound,

    /// IO error
    IO(std::io::Error),
}

impl From<std::io::Error> for StorageError {
    fn from(err: std::io::Error) -> Self {
        Self::IO(err)
    }
}
