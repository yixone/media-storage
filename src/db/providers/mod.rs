pub mod sqlite;

/// Database provider trait
pub trait DatabaseProvider: Send + Sync {}

/// Database providers abstraction
#[derive(Debug, Clone)]
pub enum Database {
    Sqlite(sqlite::SqliteDb),
}

impl std::ops::Deref for Database {
    type Target = dyn DatabaseProvider;
    fn deref(&self) -> &Self::Target {
        match self {
            Database::Sqlite(db) => db,
        }
    }
}
