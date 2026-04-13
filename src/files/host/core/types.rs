use std::path::Path;

/// Result of changing the file key in file host
#[derive(Debug, Clone, Copy)]
pub enum RenameResult {
    Renamed,
    AlreadyExists,
}

/// Result of deleting a file from a file host
#[derive(Debug, Clone, Copy)]
pub enum DeleteResult {
    Deleted,
    NotFound,
}

/// Key for a file in a file host
#[derive(Debug, Clone, PartialEq)]
pub struct FileHostKey {
    pub inner: String,
}

impl AsRef<Path> for FileHostKey {
    fn as_ref(&self) -> &Path {
        Path::new(&self.inner)
    }
}
