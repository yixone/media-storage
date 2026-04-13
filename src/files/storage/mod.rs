use std::sync::Arc;

use crate::files::host::core::traits::FileHost;

pub mod types;

/// Content-addressable storage for files
#[derive(Clone)]
pub struct Storage {
    file_host: Arc<dyn FileHost>,
}
