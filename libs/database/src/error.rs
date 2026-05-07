pub type DbResut<T> = std::result::Result<T, DatabaseError>;

pub enum DatabaseError {
    NotFound,
    Conflict,
    BrokenRelation,
    SqlxDriverError(sqlx::Error),
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
