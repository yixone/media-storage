#![allow(async_fn_in_trait)]

pub mod blob_host;
pub mod key;
pub mod result;
pub mod storage;

pub use key::StorageKey;
pub use result::StorageError;
pub use storage::ContentStorage;

pub(crate) use result::StorageResult;
