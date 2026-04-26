use crate::{create_error, error::Result};

const MAX_CURSOR: u32 = u32::MAX - (MAX_LIMIT + 1);
const MAX_LIMIT: u32 = 500;

/// Pagination Data
#[derive(Debug, Clone, Copy)]
pub struct Pagination {
    /// Pagination cursor
    pub cursor: u32,

    /// Pagination return limit
    pub limit: u32,
}

impl Pagination {
    /// Creates a new [`Pagination`] with value validation
    pub fn new_checked(cursor: u32, limit: u32) -> Result<Self> {
        if cursor > MAX_CURSOR || limit > MAX_LIMIT {
            return Err(create_error!(PaginationError));
        }
        Ok(Pagination { cursor, limit })
    }
}
