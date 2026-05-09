// Enabling Async Traits for Repository Traits
#![allow(async_fn_in_trait)]

#[macro_use]
pub mod macros;

pub mod error;
pub mod pagination;
pub mod sqlite;
pub mod traits;

pub use error::{DatabaseError, DbResut};
