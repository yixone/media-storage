#![allow(async_fn_in_trait)]

pub mod result;
pub mod sqlite;
pub mod traits;

pub use result::DatabaseError;
pub use sqlite::*;

pub(crate) use result::DbResult;
