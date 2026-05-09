use std::sync::Arc;

use ms_content_store::ContentStorage;
use ms_database::sqlite::SqliteDatabase;

#[derive(Debug, Clone)]
pub struct DataContext {
    pub db: SqliteDatabase,
    pub store: Arc<ContentStorage>,
}
