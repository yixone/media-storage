pub(crate) type DbResult<T> = std::result::Result<T, DatabaseError>;

/// Database module errors
#[derive(Debug)]
pub enum DatabaseError {
    /// No rows returned by a query
    NotFound,
    /// Violation of a unique/primary key
    UniqueConflict,
    /// SQLX Driver error
    DriverError(sqlx::Error),
    /// IO Error
    IO(std::io::Error),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::Database(e) if e.is_unique_violation() => DatabaseError::UniqueConflict,
            sqlx::Error::Io(e) => DatabaseError::IO(e),
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            _ => DatabaseError::DriverError(err),
        }
    }
}

impl From<sqlx::migrate::MigrateError> for DatabaseError {
    fn from(err: sqlx::migrate::MigrateError) -> Self {
        DatabaseError::DriverError(err.into())
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(err: std::io::Error) -> Self {
        DatabaseError::IO(err)
    }
}
