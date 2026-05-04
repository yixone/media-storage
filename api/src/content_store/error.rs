/// Content storage error
#[derive(Debug)]
pub enum StorageError {
    /// The blob with the specified key
    /// was not found in the storage
    NotFound,

    /// The key does not point to a blob
    NotABlob,
    /// The key contains invalid characters or
    /// is not the correct length
    InvalidKey,

    /// The blob size exceeds the maximum
    /// storage item size
    BlobTooLarge,

    /// IO errors
    Io(std::io::Error),
}
