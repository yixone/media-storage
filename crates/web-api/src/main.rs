use actix_cors::Cors;
use actix_web::{App, HttpServer, web::Data};

use asset_shelf_database::SqliteDatabase;
use asset_shelf_result::error::AppResult;
use asset_shelf_storage::{ContentStorage, blob_host::fs::FsBlobHost};
use asset_shelf_web_api::{
    di::{DataContext, ServerContext},
    routes,
};

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

    let data_ctx = Data::new(DataContext::new(db, store));
    let server_ctx = Data::new(ServerContext::new());

    HttpServer::new(move || {
        App::new()
            .wrap(Cors::permissive())
            .configure(routes::config)
            .app_data(data_ctx.clone())
            .app_data(server_ctx.clone())
    })
    .bind(LISTEN_ADDR)?
    .run()
    .await?;

    Ok(())
}
