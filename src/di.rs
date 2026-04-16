use crate::{db::providers::Database, files::storage::Storage};

/// Context for working with data
#[derive(Debug, Clone)]
pub struct DataContext {
    /// Database provider
    pub db: Database,
    /// Storage provider
    pub store: Storage,
}

/// Context for sending notifications
#[derive(Debug, Clone)]
pub struct NotificationsContext {}
