// Enabling Async Traits for Repository Traits
#![allow(async_fn_in_trait)]

pub mod error;
pub mod sqlite;
pub mod traits;

pub use error::{DatabaseError, DbResut};
