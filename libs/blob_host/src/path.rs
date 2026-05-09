use std::path::Path;

#[derive(Debug, Clone)]
pub struct BlobPath {
    pub inner: String,
}

impl BlobPath {
    pub fn new(path: impl Into<String>) -> Self {
        BlobPath { inner: path.into() }
    }
}

impl AsRef<Path> for BlobPath {
    fn as_ref(&self) -> &Path {
        Path::new(&self.inner)
    }
}
