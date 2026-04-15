use std::ops::Deref;

/// Database Provider master-trait
pub trait DatabaseProvider: Send + Sync {}

/// Database providers
#[derive(Debug, Clone)]
pub enum Database {
    /// SQLite database
    Sqlite(),
}

impl Deref for Database {
    type Target = dyn DatabaseProvider;
    fn deref(&self) -> &Self::Target {
        match self {
            Database::Sqlite() => todo!(),
        }
    }
}
