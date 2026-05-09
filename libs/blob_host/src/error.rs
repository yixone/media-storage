#[derive(Debug)]
pub enum BlobHostError {
    InvalidBlobPath,
    BlobPathConflict,
    Io(std::io::Error),
}

impl From<std::io::Error> for BlobHostError {
    fn from(e: std::io::Error) -> Self {
        BlobHostError::Io(e)
    }
}
