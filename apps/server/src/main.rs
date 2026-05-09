use std::sync::Arc;

use actix_web::web::Data;
use ms_blob_host::BlobHost;
use ms_content_store::ContentStorage;
use ms_database::sqlite::SqliteDatabase;
use ms_server::{di::DataContext, error::AppResult};

const DB_FILE: &str = "data/storage.db";

const BLOB_HOST_DIR: &str = "data/blobs";
const MAX_BLOB_SIZE: usize = 512 * 1024 * 1024;

const LISTEN_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .init();

    let db = SqliteDatabase::open(DB_FILE).await?;
    db.migrate().await?;

    let blob_host = BlobHost::mount_fs(BLOB_HOST_DIR)?;
    let store = ContentStorage::new(blob_host, MAX_BLOB_SIZE);

    let ctx = Data::new(DataContext {
        db,
        store: Arc::new(store),
    });

    Ok(())
}
