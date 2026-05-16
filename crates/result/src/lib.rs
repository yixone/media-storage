pub mod error;
pub mod macros;

#[cfg(feature = "actix")]
pub mod actix;

pub use error::AppError;
