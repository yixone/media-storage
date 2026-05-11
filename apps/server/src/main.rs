use std::sync::Arc;

use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};

use shelf_blob_host::BlobHost;
use shelf_content_store::ContentStorage;
use shelf_database::sqlite::SqliteDatabase;
use shelf_server::{
    bg::media::MediaWorker,
    di::{DataContext, MessagesContext},
    error::AppResult,
    routes,
};

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

    let media_worker = MediaWorker::new(ctx.clone().into_inner(), 128);
    let media_worker_msgs = media_worker.spawn().await;

    let messages_ctx = Data::new(MessagesContext { media_worker_msgs });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .configure(routes::config)
            .app_data(ctx.clone())
            .app_data(messages_ctx.clone())
    })
    .bind(LISTEN_ADDR)?
    .run()
    .await?;

    Ok(())
}
