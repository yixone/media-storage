use std::sync::Arc;

use crate::storage::core::traits::FileHost;

/// Content Addressable Storage
#[derive(Clone)]
pub struct CAStorage {
    file_host: Arc<dyn FileHost>,
    max_file_size: usize,
}

impl CAStorage {}
