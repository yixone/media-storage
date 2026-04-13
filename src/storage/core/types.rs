/// Result of changing the file key in the storage
#[derive(Debug, Clone, Copy)]
pub enum RenameResult {
    Renamed,
    AlreadyExists,
}

/// Result of deleting a file from storage
#[derive(Debug, Clone, Copy)]
pub enum DeleteResult {
    Deleted,
    NotFound,
}

/// Key for the blob in the storage
#[derive(Debug, Clone, PartialEq)]
pub struct StorageKey {
    inner: String,
}
