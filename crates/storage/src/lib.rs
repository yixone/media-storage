#![allow(async_fn_in_trait)]

pub mod blob_host;
pub mod key;
pub mod storage;

pub use key::StorageKey;
pub use storage::ContentStorage;
