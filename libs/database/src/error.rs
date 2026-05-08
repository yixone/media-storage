pub type DbResut<T> = std::result::Result<T, DatabaseError>;

pub enum DatabaseError {
    NotFound,
    Conflict,
    BrokenRelation,
    SqlxDriverError(sqlx::Error),
    SqlxMigratorError(sqlx::migrate::MigrateError),
    IoError(std::io::Error),
}

impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        use sqlx::error::ErrorKind;

        match err {
            sqlx::Error::Database(e) if e.kind() == ErrorKind::UniqueViolation => {
                DatabaseError::Conflict
            }
            sqlx::Error::Database(e) if e.kind() == ErrorKind::ForeignKeyViolation => {
                DatabaseError::BrokenRelation
            }
            sqlx::Error::RowNotFound => DatabaseError::NotFound,
            _ => DatabaseError::SqlxDriverError(err),
        }
    }
}

impl From<std::io::Error> for DatabaseError {
    fn from(e: std::io::Error) -> Self {
        DatabaseError::IoError(e)
    }
}

impl From<sqlx::migrate::MigrateError> for DatabaseError {
    fn from(e: sqlx::migrate::MigrateError) -> Self {
        DatabaseError::SqlxMigratorError(e)
    }
}
