use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};

use asset_shelf_database::SqliteDatabase;
use asset_shelf_result::error::AppResult;
use asset_shelf_storage::{ContentStorage, blob_host::fs::FsBlobHost};
use asset_shelf_web_api::{
    bg::media_worker::MediaWorker,
    di::{DataContext, MessagesContext, ServerContext},
    routes,
};
use chrono::Utc;

const LISTEN_ADDR: &str = "0.0.0.0:8080";

#[tokio::main]
async fn main() -> AppResult<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .with_target(false)
        .init();

    let blob_host = FsBlobHost::new("data/blobs")?;
    let store = ContentStorage::new(blob_host, 512 * 1024 * 1024);

    let db = SqliteDatabase::open("data/storage.db").await?;
    db.migrate().await?;

    let data_ctx = Data::new(DataContext { db, store });
    let server_ctx = Data::new(ServerContext {
        runned_at: Utc::now(),
    });

    let media_worker = MediaWorker::new(data_ctx.clone().into_inner());
    let media_worker_sender = media_worker.spawn();

    let messages_ctx = Data::new(MessagesContext {
        media_worker_sender,
    });

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .configure(routes::config)
            .app_data(data_ctx.clone())
            .app_data(server_ctx.clone())
            .app_data(messages_ctx.clone())
    })
    .bind(LISTEN_ADDR)?
    .run()
    .await?;

    Ok(())
}
